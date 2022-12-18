pub mod utils {
    use crate::PrintController;
    use crate::Todo;
    use crate::TodoState;
    use crate::_Todo;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn get_file_env() -> String {
        let mut env = String::new();
        let default_path = "~/.local/share/todo/todos.todo".to_string();
        let _env = std::env::var("TODO_FILE_LOCATION");
        match _env {
            Ok(e) => {
                let l = e.trim().len();
                if l != 0 {
                    env = e;
                } else {
                    env = default_path;
                }
            }
            Err(_) => env = default_path,
        }
        if env.starts_with("~") {
            return dirs::home_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                + &env[1..];
        }
        return env;
    }

    pub fn load_from_file(
        file_path: &str,
        todo_container: &mut Todo,
        controller: &mut PrintController,
    ) -> Result<(), std::io::Error> {
        let file: File;
        if let Ok(_file) = File::open(file_path) {
            file = _file
        } else {
            let mut path = file_path.split('/').collect::<Vec<&str>>();
            path.pop();
            std::fs::create_dir_all(path.join("/"))?;
            file = File::create(file_path)?;
        }
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let _line = line?.clone();
            let active = {
                if _line.starts_with("Not Done: ") {
                    TodoState::NotDone
                } else if _line.starts_with("Done: ") {
                    TodoState::Done
                } else {
                    TodoState::InProgress
                }
            };
            if _line.starts_with("Filter: ") {
                if &_line[8..] == "Other" {
                    controller.set_state(TodoState::Other);
                } else if &_line[8..] == "NotDone" {
                    controller.set_state(TodoState::NotDone);
                } else if &_line[8..] == "InProgress" {
                    controller.set_state(TodoState::InProgress);
                } else {
                    controller.set_state(TodoState::Done);
                }
            } else {
                todo_container.add_todo(_Todo {
                    id: todo_container.todos.len() as i32,
                    content: _line[active.to_string().len() + 2..].to_string(),
                    active,
                });
            }
        }
        Ok(())
    }
}
