pub mod file;
pub mod print;
pub mod todo;

#[derive(std::fmt::Debug, Copy, Clone, PartialEq)]
pub enum TodoState {
    Other,
    NotDone,
    InProgress,
    Done,
}

impl std::fmt::Display for TodoState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self);
        Ok(())
    }
}

#[derive(Clone)]
pub struct _Todo {
    pub id: i32,
    pub active: TodoState,
    pub content: String,
}

//==============================//
//    TODO STRUCT CONTAINER     //
//==============================//

pub struct Todo {
    pub todos: Vec<_Todo>,
    pub history: Vec<_Todo>,
}

pub struct PrintController {
    pub tab: TodoState,
}
