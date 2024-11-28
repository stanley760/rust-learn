use todolist::routers::router;

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();
    
    let bind = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let app = router::route().await.unwrap();
    axum::serve(bind, app)
    .await
    .unwrap();
}
