use std::sync::{Arc, Mutex};
use axum::{http::{StatusCode, Uri}, Router};
use axum::routing::{get, post};
use rusqlite::Connection;
use tower_http::trace::TraceLayer;
use crate::repository::opts::{create, delete, list, update};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

pub async fn route() -> anyhow::Result<Router>{
    let conn = crate::config::connectdb::connect_db()?;
    let db = AppState {db: Arc::new(Mutex::new(conn))};
    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete", get(delete))
        .route("/update", post(update))
        .with_state(db)
        .fallback(fallback)
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}