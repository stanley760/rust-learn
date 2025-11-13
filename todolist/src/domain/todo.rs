use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTodo {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTodo {
    pub id: i32,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultWrapper<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}
