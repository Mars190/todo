use core::fmt;
use crate::todo::Todo;

pub struct AppState {
    mode: Mode,
    todos: Vec<Todo>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mode: Mode::Main,
            todos: Vec::new(),
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }
}

#[derive(Debug)]
pub enum Mode {
    Main,
    Search,
    Create,
    Edit,
    Delete,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Main => write!(f, "Main Mode"),
            Mode::Search => write!(f, "Search Mode"),
            Mode::Create => write!(f, "Create Mode"),
            Mode::Edit => write!(f, "Edit Mode"),
            Mode::Delete => write!(f, "Delete Mode"),
        }
    }
}

impl fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TodoService{{ mode: {}, todos: {:?} }}", self.mode, self.todos)
    }
}