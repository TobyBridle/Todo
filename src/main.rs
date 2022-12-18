use ncurses::*;

use std::cmp::min;

pub mod utils;
pub use crate::{
    file::utils::{get_file_env, load_from_file},
    print::utils::print_controls,
    utils::*,
};

//==============================//
//          MAIN METHOD         //
//==============================//

pub const NO_SELECT: i16 = 0;
pub const SELECT: i16 = 1;
pub const MENU_BAR: i16 = 2;
pub const COMPLETED: i16 = 3;

fn navigate_up(cursor_position: usize) -> usize {
    if cursor_position == 0 {
        return 0;
    };
    return cursor_position - 1;
}

fn navigate_down(todo_len: usize, cursor_position: usize) -> usize {
    if todo_len == 0 {
        return 0;
    };
    if todo_len == cursor_position + 1 {
        return cursor_position;
    };
    return cursor_position + 1;
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

fn main() {
    let file_path: &str = &get_file_env();

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

    let mut cursor_position: usize = 0;
    let mut todos = Todo {
        todos: Vec::new(),
        history: Vec::new(),
    };

    let mut controller = PrintController::new();

    load_from_file(file_path, &mut todos, &mut controller);

    let mut key;
    while run {
        mv(0, 0);

        let spacing = print_controls(&controller);
        let mut index: i32 = -1;
        let mut mapping: Vec<usize> = Vec::new();

        if todos.todos.len() == 0 {
            mv(-1 + spacing as i32, 0);
            attron(COLOR_PAIR(SELECT));
            addstr("You have no TODOs, try creating some by pressing `a`");
            attroff(COLOR_PAIR(SELECT));
        }

        let has_overflowed: bool = if cursor_position as i32 >= LINES() - spacing as i32 {
            true
        } else {
            false
        };
        // We want start to be either 0 or the top of the screen if we have overflowed
        // We want end to be either length of todos or the max amount of todos that will fit on the
        // screen

        let start: usize = if !has_overflowed {
            0
        } else {
            LINES() as usize - spacing
        };
        let end: usize = if !has_overflowed {
            min(todos.todos.len() as i32, LINES() - spacing as i32) as usize
        } else {
            (LINES() - spacing as i32 + cursor_position as i32 - 1) as usize
        };

        for t in 0..todos.todos.len() {
            let todo = &todos.todos[t as usize];
            if todo.active != controller.tab && controller.tab != TodoState::Other {
                continue;
            }

            index += 1;
            if start as i32 > index || end < index as usize {
                continue;
            }

            mapping.push(todo.id as usize);
            let state = if matches!(todo.active, TodoState::NotDone) {
                "- [ ]\t"
            } else if matches!(todo.active, TodoState::InProgress) {
                "- [-]\t"
            } else {
                "- [x]\t"
            };
            let hl: i16 = if index as usize == cursor_position {
                SELECT
            } else if matches!(todo.active, TodoState::Done) {
                COMPLETED
            } else {
                NO_SELECT
            };
            if !has_overflowed {
                mv(index + spacing as i32 - 1, 0); // Set Cursor to Beginning of next Line
            } else {
                mv((t - start + spacing) as i32 - 1, 0);
            }

            attron(COLOR_PAIR(hl));
            addstr(state);
            addstr(&todo.content.trim());
            attroff(COLOR_PAIR(hl));
        }
        refresh();

        key = getch();

        match key as u8 as char {
            'k' => cursor_position = navigate_up(cursor_position),
            'j' => cursor_position = navigate_down(index as usize + 1, cursor_position),
            '\n' => {
                cursor_position = todos.toggle_todo(
                    mapping.len(),
                    if mapping.len() > cursor_position {
                        mapping[cursor_position] as i32
                    } else {
                        -1
                    },
                    &controller,
                    cursor_position,
                );
            }
            'a' => {
                cursor_position = if todos.add_todo_prompt() {
                    mapping.len()
                } else {
                    cursor_position
                }
            }
            'e' => {
                cursor_position = todos.edit_todo(
                    mapping.len(),
                    if mapping.len() > cursor_position {
                        mapping[cursor_position] as i32
                    } else {
                        -1
                    },
                    cursor_position,
                )
            }
            'r' => {
                cursor_position = todos.remove_todo(
                    mapping.len(),
                    if mapping.len() > cursor_position {
                        mapping[cursor_position] as i32
                    } else {
                        -1
                    },
                    cursor_position,
                )
            }
            'd' => {
                cursor_position = todos.set_in_progress(
                    mapping.len(),
                    if mapping.len() > cursor_position {
                        mapping[cursor_position] as i32
                    } else {
                        -1
                    },
                    &controller,
                    cursor_position,
                )
            }
            'u' => {
                let pos = todos.undo(mapping.len());
                cursor_position = if pos == -1 {
                    cursor_position
                } else {
                    pos as usize
                }
            }
            '\t' => {
                controller.cycle_tab();
                cursor_position = 0;
            }
            's' => todos.save(file_path, &controller),
            'q' => {
                run = false;
            }
            _ => {
                continue;
            }
        }
        todos.save(file_path, &controller);
        endwin();
    }
}