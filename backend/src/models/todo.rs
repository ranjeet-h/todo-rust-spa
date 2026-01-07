use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

/// Represents a Todo item in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")] // Optional to handle legacy data if needed
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        let now = DateTime::now();
        Self {
            id: None,
            title,
            completed: false,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

/// DTO for creating a new todo
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

/// DTO for updating a todo
#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

/// Response DTO with string ID for JSON serialization
#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id.map(|id| id.to_hex()).unwrap_or_default(),
            title: todo.title,
            completed: todo.completed,
            // Convert BSON DateTime to RFC3339 string for JSON response
            created_at: todo.created_at
                .map(|dt| dt.try_to_rfc3339_string().unwrap_or_default())
                .unwrap_or_default(),
            updated_at: todo.updated_at
                .map(|dt| dt.try_to_rfc3339_string().unwrap_or_default())
                .unwrap_or_default(),
        }
    }
}
