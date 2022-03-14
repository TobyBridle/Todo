use ncurses::*;

use std::io::{Write, BufReader, BufRead, BufWriter};
use std::fs::File;

//==============================//
//    SINGULAR TODO STRUCT      //
//==============================//

#[derive(std::fmt::Debug, Copy, Clone)]
enum TODOS
{
   NotDone,
   Done,
   InProgress,
}

impl std::fmt::Display for TODOS
{
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
   {
      write!(f, "{:?}", self);
      Ok(())
   }
}

#[derive(Clone)]
struct _Todo {
   id: i32,
   active: TODOS,
   content: String,
}

impl _Todo
{
   fn new(content: String) -> _Todo {
      _Todo {
         id: -1,
         active: TODOS::NotDone,
         content,
      }
   }
}

//==============================//
//    TODO STRUCT CONTAINER     //
//==============================//

struct Todo
{
   todos: Vec<_Todo>,
   history: Vec<_Todo>,
}

impl Todo {

   pub fn add_todo(&mut self, mut todo: _Todo)
   {
      todo.id = self.todos.len() as i32;
      self.todos.push(todo);
   }
   
   pub fn add_todo_prompt(&mut self)
   {
      clear();
      echo();
      mv(0, 0);
      attron(COLOR_PAIR(SELECT));
      addstr("Add a Todo: (ESC to exit)");
      attroff(COLOR_PAIR(SELECT));
      mv(1, 0);
      
      let mut content: String = String::new();
      let mut ch: char = getch() as u8 as char;
      
      while (ch != '\n' && ch as u8 != 27)
      {
         content.push(ch);
         ch = getch() as u8 as char;
      }
      
      if content.trim().len() > 0 { self.add_todo(_Todo::new(content.trim().to_string())) };
      noecho();
   }
   
   fn toggle_todo(&mut self, id: i32)
   {
      if self.todos.len() == 0 { return; }
      let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
      if matches!(self.todos[index].active, TODOS::Done) {self.todos[index].active = TODOS::NotDone} else { self.todos[index].active = TODOS::Done};
   }
   
   fn set_in_progress(&mut self, id: i32)
   {
      if self.todos.len() == 0 { return; }
      let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
      if(matches!(self.todos[index].active, TODOS::InProgress)) { self.todos[index].active = TODOS::NotDone} else { self.todos[index].active = TODOS::InProgress };
   }
   
   fn remove_todo(&mut self, id: usize) -> bool
   {
      if self.todos.len() == 0 { return false }

      self.history.push(self.todos[id].clone());
      self.todos.remove(id);
      clear();
      return true;
   }
   
   fn undo(&mut self) -> i32
   {
      if(self.history.len() == 0) { return -1 }
      let prev_todo: _Todo = self.history.pop().unwrap();
      if self.todos.len() < prev_todo.id as usize {self.todos.push(prev_todo)} else {self.todos.insert(prev_todo.id as usize, prev_todo)};

      clear();
      return (self.todos.len() - 1) as i32;
   }
   
   fn save(&self, file_path: &str)
   {
      let file = File::create(file_path).expect("Could not write to file!");
      let mut writer = BufWriter::new(file);
      
      for todo in self.todos.clone()
      {
         let active = 
            if matches!(todo.active, TODOS::Done) {"Done: "}
            else if matches!(todo.active, TODOS::NotDone) {"NOT Done: "}
            else {"IN PROGRESS: "};
         writer.write(format!("{} {}\n", active, todo.content).as_bytes()).unwrap();
      }
      writer.flush().expect("Could not write to file!");
   }
}
   
//==============================//
//          MAIN METHOD         //
//==============================//

pub const FILE_PATH: &str = "/Users/tobybridle/.config/todo/todos.todo";

pub const NO_SELECT: i16 = 0;
pub const SELECT: i16 = 1;
pub const MENU_BAR: i16 = 2;
pub const COMPLETED: i16 = 3;

pub const MENU_BAR_SIZE: usize = 3;

fn load_from_file(file_path: &str, todo_container: &mut Todo) -> Result<(), std::io::Error>
{
   let file = File::open(file_path)?;
   let reader = BufReader::new(file);
   
   for line in reader.lines() {
      let _line = line?.clone();
      let active = 
      {
         if _line.starts_with("NOT Done: ") { TODOS::NotDone }
         else if _line.starts_with("Done: ") { TODOS::Done }
         else { TODOS::InProgress }
      };
      
      todo_container.add_todo(_Todo { id: todo_container.todos.len() as i32, content: _line[active.to_string().len()+2..].to_string(), active });
   }
   
   Ok(())
}

fn navigate_up(todo_len: usize, cursor_position: usize) -> usize
{
   if todo_len == 0 { return 0 };
   if todo_len- 1 == cursor_position { return cursor_position };
   return cursor_position + 1;
}

fn print_controls()
{
   attron(COLOR_PAIR(MENU_BAR));
   addstr("Toggle Todo State (Enter)\tMove Down (J)\tMove Up (K)\tQuit (Q)\tUndo most recent delete (U)");
   mv(1, 0);
   addstr("Add New Todo (A)\t\tRemove Todo (R)\t\t\tSet Todo to In-Progress (D)");
   attroff(COLOR_PAIR(MENU_BAR));
}

fn main()
{
   initscr();
   noecho();
   curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
   
   start_color();
   init_pair(NO_SELECT, COLOR_WHITE, COLOR_BLACK);
   init_pair(SELECT, COLOR_BLACK, COLOR_WHITE);
   init_pair(MENU_BAR, COLOR_BLACK, COLOR_GREEN);
   init_pair(COMPLETED, COLOR_BLACK, COLOR_CYAN);
   
   let mut run = true;
   
   let mut cursor_position = 0;
   let mut todos = Todo {
      todos: Vec::new(),
      history: Vec::new(),
   };
   
   load_from_file(FILE_PATH, &mut todos);

   let mut key;
   while run
   {
      mv(0, 0);
      print_controls();
      if todos.todos.len() == 0
      {
         mv(MENU_BAR_SIZE as i32, 0);
         attron(COLOR_PAIR(SELECT));
         addstr("You have no TODOs, try creating some by pressing `a`");
         attroff(COLOR_PAIR(SELECT));
      }
      for (index, todo) in todos.todos.iter().enumerate()
      {
         let state = if matches!(todo.active, TODOS::NotDone) {"- [ ]\t"} else if matches!(todo.active, TODOS::InProgress) {"- [-]\t"} else {"- [x]\t"};
         let hl: i16 = if index == cursor_position {SELECT} else if matches!(todo.active, TODOS::Done) {COMPLETED} else {NO_SELECT};
         mv((index + MENU_BAR_SIZE) as i32, 0); // Set Cursor to Beginning of next Line
         
         attron(COLOR_PAIR(hl));
         addstr(state);
         addstr(&todo.content);
         attroff(COLOR_PAIR(hl));
      }
      
      key = getch();
      refresh();
      match key as u8 as char
      {
         'k' => { cursor_position = if cursor_position == 0 {0} else {cursor_position - 1}}
         'j' => { cursor_position = navigate_up(todos.todos.len(), cursor_position)}
         '\n' => { todos.toggle_todo(cursor_position as i32) }
         'a' => { todos.add_todo_prompt() }
         'r' => { if todos.remove_todo(cursor_position) {cursor_position = if cursor_position == 0 {0} else {cursor_position - 1}}}
         'd' => { todos.set_in_progress(cursor_position as i32) }
         'u' => { let pos = todos.undo(); cursor_position  = if pos == -1 {cursor_position} else {pos as usize}}
         'q' => { run = false; },
         's' => { todos.save(FILE_PATH) }
         _ => { continue; }
      }
      todos.save(FILE_PATH);
      endwin();
   }
   
}