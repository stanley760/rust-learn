use axum::{response::IntoResponse, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::config::dbconnector::AppState;
use crate::domain::todo::{CreateTodo, ResultWrapper, Todo};


pub async fn create() {}

pub async fn list(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state.pool.get().await {
        Ok(db) => {
            todo!
            (StatusCode::OK, Json(ResultWrapper { code: 0, message: "ok".to_string(), data:None }))
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResultWrapper { code: 500, message: e.to_string(), data: None }))
        }
    }
}

async fn todo_delete() -> impl IntoResponse {}

async fn todo_create() -> impl IntoResponse {}