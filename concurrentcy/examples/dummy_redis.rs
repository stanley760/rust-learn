
use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:6379";
   //  let listener = TcpListener::bind(addr).await?;
    info!("Prdis: Listening on {}", addr);
    Ok(())
}