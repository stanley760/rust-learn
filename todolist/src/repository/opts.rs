use crate::domain::todo::{CreateTodo, ResultWrapper, Todo, UpdateTodo};
use crate::routers::router::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use rusqlite::params;

pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    tracing::info!("todo list info: {:?} had been create", input);
    let conn = state.db.lock().unwrap();
    let rowid = conn
        .execute(
            "INSERT INTO todo (description, completed) VALUES ($1, FALSE) RETURNING id",
            [&input.description],
        )
        .map_err(|e| {
            eprintln!("Error creating todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResultWrapper::<usize> {
                    code: 500,
                    message: e.to_string(),
                    data: None,
                }),
            )
        })
        .unwrap();

    (
        StatusCode::OK,
        Json(ResultWrapper {
            code: 0,
            message: "ok".to_string(),
            data: Some(rowid),
        }),
    )
}

pub async fn update(
    State(state): State<AppState>,
    Json(input): Json<UpdateTodo>,
) -> impl IntoResponse {
    tracing::info!("todo list info: {:?} had been update", input);
    let conn = state.db.lock().unwrap();
    let id = conn
        .execute(
            "UPDATE todo SET description = $1, completed = $2 WHERE id = $3",
            params![&input.description, &input.completed, &input.id],
        )
        .map_err(|e| {
            eprintln!("Error updating todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResultWrapper::<usize> {
                    code: 500,
                    message: e.to_string(),
                    data: None,
                }),
            )
        })
        .unwrap();
    (
        StatusCode::OK,
        Json(ResultWrapper {
            code: 0,
            message: "ok".to_string(),
            data: Some(id),
        }),
    )
}

pub async fn delete(State(state): State<AppState>, Json(id): Json<i32>) -> impl IntoResponse {
    tracing::info!("todo list id {} had been deleted", id);
    let conn = state.db.lock().unwrap();
    let id = conn
        .execute("DELETE FROM todo WHERE id = $1", [&id])
        .map_err(|e| {
            eprintln!("Error deleting todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResultWrapper::<usize> {
                    code: 500,
                    message: e.to_string(),
                    data: None,
                }),
            )
        })
        .unwrap();
    (
        StatusCode::OK,
        Json(ResultWrapper {
            code: 0,
            message: "ok".to_string(),
            data: Some(id),
        }),
    )
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM todo")
        .map_err(|e| {
            eprintln!("Error preparing statement: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResultWrapper::<Vec<Todo>> {
                    code: 500,
                    message: e.to_string(),
                    data: None,
                }),
            )
        })
        .unwrap();

    let result = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .unwrap();
    let todos: Vec<Todo> = result.map(|item| item.unwrap()).collect();

    (
        StatusCode::OK,
        Json(ResultWrapper::<Vec<Todo>> {
            code: 0,
            message: "ok".to_string(),
            data: Some(todos),
        }),
    )
}
