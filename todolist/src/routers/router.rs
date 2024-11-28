
use axum::{http::{StatusCode, Uri}, routing::{get, post}, Router};
use tower_http::trace::TraceLayer;

use crate::handler::opts;


pub async fn route() -> anyhow::Result<Router>{
    let app = Router::new()
        .route("/", get(opts::list))
        .route("/create", post(opts::create))
        .fallback(fallback)
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}