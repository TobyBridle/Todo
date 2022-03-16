use ncurses::*; 
use dialoguer::Input;

use std::cmp::{min, max};
use std::io::{Write, BufWriter};
use std::fs::File;

mod utils;
use crate::utils::file::utils::*;
use crate::utils::print::utils::*;

//==============================//
//    SINGULAR TODO STRUCT      //
//==============================//

#[derive(std::fmt::Debug, Copy, Clone, PartialEq)]
pub enum TodoState
{
   Other,
   NotDone,
   InProgress,
   Done,
}

impl std::fmt::Display for TodoState
{
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
   {
      write!(f, "{:?}", self);
      Ok(())
   }
}

#[derive(Clone)]
pub struct _Todo {
   id: i32,
   active: TodoState,
   content: String,
}

impl _Todo
{
   fn new(content: String) -> _Todo {
      _Todo {
         id: -1,
         active: TodoState::NotDone,
         content,
      }
   }
}

//==============================//
//    TODO STRUCT CONTAINER     //
//==============================//

pub struct Todo
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
   
   pub fn add_todo_prompt(&mut self) -> bool
   {
      clear();
      echo();
      curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
      mv(0, 0);

      attron(COLOR_PAIR(SELECT));
      addstr("Add a Todo: (Enter without text to exit)");
      attroff(COLOR_PAIR(SELECT));
      mv(1, 0);
      refresh();
      
      let content: String = Input::new()
                             .allow_empty(true)
                             .interact_text()
                             .unwrap();

      noecho();
      curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

      if content.trim().len() > 0
      {
         self.add_todo(_Todo::new(content.trim().to_string()));
         return true;
      };
      return false;
   }
   
   fn edit_todo(&mut self, length: usize, id: i32, cursor_position: usize) -> usize
   {
      if length == 0 || id < 0 { return 0; }
      let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
      let original_todo = &self.todos[index];
      
      clear();
      echo();
      curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
      mv(0, 0);

      attron(COLOR_PAIR(SELECT));
      addstr("Add a Todo: (Enter without text to exit)");
      attroff(COLOR_PAIR(SELECT));
      mv(1, 0);
      refresh();
      
      let edited_content: String = Input::new()
                                   .allow_empty(true)
                                   .with_initial_text(original_todo.content.to_owned())
                                   .interact_text()
                                   .unwrap();

      noecho();
      curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

      if edited_content.trim() == original_todo.content.trim() { return cursor_position; }
      
      if edited_content.trim().len() == 0 { return self.remove_todo(length, id, cursor_position); }
      
      self.todos[index].content = edited_content;
      return cursor_position;
      
   }
   
   fn toggle_todo(&mut self, length: usize, id: i32, controller: &PrintController, cursor_position: usize) -> usize
   {
      if length == 0 || id < 0 { return 0; }
      let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
      if matches!(self.todos[index].active, TodoState::Done) {self.todos[index].active = TodoState::NotDone} else { self.todos[index].active = TodoState::Done};

      if (controller.tab == TodoState::Other) { return cursor_position }
      else if cursor_position + 1 == length && cursor_position > 0 { return cursor_position - 1 }
      return cursor_position;
   }
   
   fn set_in_progress(&mut self, length: usize, id: i32, controller: &PrintController, cursor_position: usize) -> usize
   {
      if length == 0 || id < 0 { return 0; }
      let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
      if(matches!(self.todos[index].active, TodoState::InProgress)) { self.todos[index].active = TodoState::NotDone} else { self.todos[index].active = TodoState::InProgress };

      if (controller.tab == TodoState::Other) { return cursor_position }
      else if cursor_position + 1 == length && cursor_position > 0 { return cursor_position - 1 }
      return cursor_position;
   }
   
   fn remove_todo(&mut self, length: usize, id: i32, cursor_position: usize) -> usize
   {
      if length == 0 || id < 0 { return 0; }

      self.history.push(self.todos[id as usize].clone());
      self.todos.remove(id as usize);
      
      for i in (id as usize)..self.todos.len()
      {
         self.todos[i].id -= 1;
      }
      
      clear();

      if cursor_position + 1 == length && cursor_position > 0 { return cursor_position - 1 }
      return cursor_position;
   }
   
   fn undo(&mut self, length: usize) -> i32
   {
      if(self.history.len() == 0) { return -1 }
      let mut prev_todo: _Todo = self.history.pop().unwrap();
      prev_todo.id = self.todos.len() as i32;
      self.todos.push(prev_todo);

      clear();
      return length as i32;
   }
   
   fn save(&self, file_path: &str)
   {
      let file = File::create(file_path).expect("Could not write to file!");
      let mut writer = BufWriter::new(file);
      
      for todo in self.todos.clone()
      {
         let active = 
            if matches!(todo.active, TodoState::Done) {"Done: "}
            else if matches!(todo.active, TodoState::NotDone) {"Not Done: "}
            else {"In Progress: "};
         writer.write(format!("{}{}\n", active, todo.content.trim()).as_bytes()).unwrap();
      }
      writer.flush().expect("Could not write to file!");
   }
}
   
//==============================//
//          MAIN METHOD         //
//==============================//

pub const NO_SELECT: i16 = 0;
pub const SELECT: i16 = 1;
pub const MENU_BAR: i16 = 2;
pub const COMPLETED: i16 = 3;

fn navigate_up(todo_len: usize, cursor_position: usize) -> usize
{
   if todo_len == 0 || cursor_position == 0 { return 0 };
   return cursor_position - 1;
}

fn navigate_down(todo_len: usize, cursor_position: usize) -> usize
{
   if todo_len == 0 { return 0 };
   if todo_len == cursor_position + 1 { return cursor_position };
   return cursor_position + 1;
}

fn main()
{
   let FILE_PATH: &str = &get_file_env();

   initscr();
   noecho();
   keypad(stdscr(), true);
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
   
   let mut controller = PrintController::new();
   
   load_from_file(FILE_PATH, &mut todos);

   let mut key;
   while run
   {
      mv(0, 0);
      
      let spacing = print_controls(&controller);
      let mut index: i32 = -1;
      let mut mapping: Vec<usize> = Vec::new();
      
      if todos.todos.len() == 0
      {
         mv(-1 + spacing as i32, 0);
         attron(COLOR_PAIR(SELECT));
         addstr("You have no TODOs, try creating some by pressing `a`");
         attroff(COLOR_PAIR(SELECT));
      }
      
      for t in 0..min((LINES()) - spacing as i32, todos.todos.len() as i32)
      {
         let todo = &todos.todos[t as usize];
         if todo.active != controller.tab && controller.tab != TodoState::Other { continue; }
         index += 1;
         mapping.push(todo.id as usize);
         let state = if matches!(todo.active, TodoState::NotDone) {"- [ ]\t"} else if matches!(todo.active, TodoState::InProgress) {"- [-]\t"} else {"- [x]\t"};
         let hl: i16 = if index as usize == cursor_position {SELECT} else if matches!(todo.active, TodoState::Done) {COMPLETED} else {NO_SELECT};
         mv(index + spacing as i32 - 1, 0); // Set Cursor to Beginning of next Line
         
         attron(COLOR_PAIR(hl));
         addstr(state);
         addstr(&todo.content.trim());
         attroff(COLOR_PAIR(hl));
      }

      key = getch();
      refresh();
      match key as u8 as char
      {
         'k' => { cursor_position = navigate_up(mapping.len(), cursor_position)}
         'j' => { cursor_position = navigate_down(mapping.len(), cursor_position)}
         '\n' => { cursor_position = todos.toggle_todo(mapping.len(), if mapping.len() > cursor_position { mapping[cursor_position] as i32 } else { -1 }, &controller, cursor_position); }
         'a' => { cursor_position = if todos.add_todo_prompt() { mapping.len() } else { cursor_position } }
         'e' => { cursor_position = todos.edit_todo(mapping.len(), if mapping.len() > cursor_position { mapping[cursor_position] as i32 } else { -1 }, cursor_position) }
         'r' => { cursor_position = todos.remove_todo(mapping.len(), if mapping.len() > cursor_position { mapping[cursor_position] as i32} else { -1 }, cursor_position) }
         'd' => { cursor_position = todos.set_in_progress(mapping.len(), if mapping.len() > cursor_position { mapping[cursor_position] as i32} else { -1}, &controller, cursor_position)}
         'u' => { let pos = todos.undo(mapping.len()); cursor_position  = if pos == -1 {cursor_position} else {pos as usize}}
         'q' => { run = false; },
         's' => { todos.save(FILE_PATH) }
         '\t' => { controller.cycle_tab(); cursor_position = 0; }
         _ => { continue; }
      }
      todos.save(FILE_PATH);
      endwin();
   }
   
}