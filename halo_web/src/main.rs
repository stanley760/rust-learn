use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let req = Router::new().route("/", get(handle_request));

    let web_app = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("linstening on {}", web_app.local_addr().unwrap());
    axum::serve(web_app, req).await.unwrap();
}

async fn handle_request() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
