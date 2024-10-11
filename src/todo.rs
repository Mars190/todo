use uuid::Uuid;
use core::fmt;

#[derive(Clone, Debug)]
pub struct Todo {
    id: Uuid,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            title: title,
            completed: false
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}