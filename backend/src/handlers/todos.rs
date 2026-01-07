use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use std::sync::Arc;

use crate::models::{
    Todo,
    todo::{CreateTodoRequest, TodoResponse, UpdateTodoRequest},
};
use crate::AppState;

/// Helper to get the todos collection
fn get_collection(state: &AppState) -> Collection<Todo> {
    state.db.collection::<Todo>("todos")
}

/// List all todos
/// GET /api/todos
pub async fn list_todos(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let collection = get_collection(&state);

    let mut cursor = collection
        .find(doc! {})
        .sort(doc! { "created_at": -1 })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut todos: Vec<TodoResponse> = Vec::new();
    
    while cursor.advance().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))? {
        let todo = cursor.deserialize_current()
            .map_err(|e| {
                tracing::error!("Data mismatch in MongoDB: {}. This usually happens if old string-based dates are present.", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database format mismatch: {}. Please drop the 'todos' collection to reset.", e))
            })?;
        todos.push(todo.into());
    }

    Ok(Json(todos))
}

/// Get a single todo by ID
/// GET /api/todos/:id
pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let collection = get_collection(&state);

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    let todo = collection
        .find_one(doc! { "_id": object_id })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(TodoResponse::from(todo)))
}

/// Create a new todo
/// POST /api/todos
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let collection = get_collection(&state);

    if payload.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title cannot be empty".to_string()));
    }

    let todo = Todo::new(payload.title.trim().to_string());

    let result = collection
        .insert_one(&todo)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted_id = result.inserted_id.as_object_id()
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get inserted ID".to_string()))?;

    let created_todo = collection
        .find_one(doc! { "_id": inserted_id })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve created todo".to_string()))?;

    Ok((StatusCode::CREATED, Json(TodoResponse::from(created_todo))))
}

/// Update an existing todo
/// PUT /api/todos/:id
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let collection = get_collection(&state);

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    // Build update document
    let mut update_doc = doc! {
        "updated_at": mongodb::bson::DateTime::now()
    };

    if let Some(title) = &payload.title {
        if title.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Title cannot be empty".to_string()));
        }
        update_doc.insert("title", title.trim());
    }

    if let Some(completed) = payload.completed {
        update_doc.insert("completed", completed);
    }

    let result = collection
        .find_one_and_update(
            doc! { "_id": object_id },
            doc! { "$set": update_doc },
        )
        .return_document(mongodb::options::ReturnDocument::After)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(TodoResponse::from(result)))
}

/// Delete a todo
/// DELETE /api/todos/:id
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let collection = get_collection(&state);

    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    let result = collection
        .delete_one(doc! { "_id": object_id })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.deleted_count == 0 {
        return Err((StatusCode::NOT_FOUND, "Todo not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
