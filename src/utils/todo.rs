//==============================//
//    SINGULAR TODO STRUCT      //
//==============================//

use crate::utils::*;
use crate::PrintController;
use crate::SELECT;
use dialoguer::Input;
use ncurses::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

impl _Todo {
    fn new(content: String) -> _Todo {
        _Todo {
            id: -1,
            active: TodoState::NotDone,
            content,
        }
    }
}

impl Todo {
    pub fn add_todo(&mut self, mut todo: _Todo) {
        todo.id = self.todos.len() as i32;
        self.todos.push(todo);
    }

    pub fn add_todo_prompt(&mut self) -> bool {
        clear();
        echo();
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        mv(0, 0);

        attron(COLOR_PAIR(SELECT));
        addstr("Add a Todo: (Enter without text to exit)");
        attroff(COLOR_PAIR(SELECT));
        mv(1, 0);
        refresh();

        let content: String = Input::new().allow_empty(true).interact_text().unwrap();

        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        if content.trim().len() > 0 {
            self.add_todo(_Todo::new(content.trim().to_string()));
            return true;
        };
        return false;
    }

    pub fn edit_todo(&mut self, length: usize, id: i32) -> bool {
        if length == 0 || id < 0 {
            return false;
        }
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

        if edited_content.trim() == original_todo.content.trim() {
            return false;
        }

        if edited_content.trim().len() == 0 {
            self.remove_todo(length, id);
            return true;
        }

        self.todos[index].content = edited_content;
        return false
    }

    pub fn toggle_todo(
        &mut self,
        length: usize,
        id: i32,
        controller: &PrintController,
        cursor_position: usize,
    ) -> usize {
        if length == 0 || id < 0 {
            return 0;
        }
        let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
        if matches!(self.todos[index].active, TodoState::Done) {
            self.todos[index].active = TodoState::NotDone
        } else {
            self.todos[index].active = TodoState::Done
        };

        if (controller.tab == TodoState::Other) {
            return cursor_position;
        } else if cursor_position + 1 == length && cursor_position > 0 {
            return cursor_position - 1;
        }
        return cursor_position;
    }

    pub fn set_in_progress(
        &mut self,
        length: usize,
        id: i32,
        controller: &PrintController,
        cursor_position: usize,
    ) -> usize {
        if length == 0 || id < 0 {
            return 0;
        }
        let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
        if (matches!(self.todos[index].active, TodoState::InProgress)) {
            self.todos[index].active = TodoState::NotDone
        } else {
            self.todos[index].active = TodoState::InProgress
        };

        if (controller.tab == TodoState::Other) {
            return cursor_position;
        } else if cursor_position + 1 == length && cursor_position > 0 {
            return cursor_position - 1;
        }
        return cursor_position;
    }

    pub fn remove_todo(&mut self, length: usize, id: i32) {
        if length == 0 || id < 0 {
            return;
        }

        // Length is only the length of
        // the onscreen items, not the total
        self.history.push(self.todos[id as usize].clone());
        self.todos.remove(id as usize);

        for (i, todo) in self.todos.iter_mut().enumerate() {
            todo.id = i as i32;
        }

        clear();
    }

    pub fn undo(&mut self, length: usize) -> i32 {
        if self.history.len() == 0 {
            return -1;
        }
        let mut prev_todo: _Todo = self.history.pop().unwrap();
        prev_todo.id = self.todos.len() as i32;
        self.todos.push(prev_todo);

        clear();
        return length as i32;
    }

    pub fn save(&self, file_path: &str, controller: &PrintController) {
        let file = File::create(file_path)
            .expect(&format!("Could not write to file {}!", file_path).to_owned());
        let mut writer = BufWriter::new(file);

        for todo in self.todos.clone() {
            let active = if matches!(todo.active, TodoState::Done) {
                "Done: "
            } else if matches!(todo.active, TodoState::NotDone) {
                "Not Done: "
            } else {
                "In Progress: "
            };
            writer
                .write(format!("{}{}\n", active, todo.content.trim()).as_bytes())
                .unwrap();
        }
        writer.write(format!("Filter: {}\n", (*controller).tab).as_bytes());
        writer.flush().expect("Could not write to file!");
    }
}
