pub mod utils
{
   use std::fs::File;
   use std::io::{BufReader, BufRead};
   use crate::_Todo;
   use crate::Todo;
   use crate::TodoState;
   
   pub fn get_file_env() -> String
   {
      match std::env::var("TODO_FILE_LOCATION")
      {
         Ok(val) => return val,
         Err(_e) => { println!("Could not find environment variable TODO_FILE_LOCATION!"); std::process::exit(1); }
      }
   }

   pub fn load_from_file(file_path: &str, todo_container: &mut Todo) -> Result<(), std::io::Error>
   {
      let file = File::open(file_path)?;
      let reader = BufReader::new(file);
      
      for line in reader.lines() {
         let _line = line?.clone();
         let active = 
         {
            if _line.starts_with("Not Done: ") { TodoState::NotDone }
            else if _line.starts_with("Done: ") { TodoState::Done }
            else { TodoState::InProgress }
         };
         todo_container.add_todo(_Todo { id: todo_container.todos.len() as i32, content: _line[active.to_string().len()+2..].to_string(), active });
      }
      Ok(())
   }
}