use anyhow::Result;
use async_prost::AsyncProstStream;
use tracing::info;
use kv_server::{CommandRequest, CommandResponse, MemTable, Service};
use futures::prelude::*;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service: Service = Service::new(MemTable::new());
    let addr = "localhost:9527";

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream = 
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(msg)) = stream.next().await {
                let res = svc.execute(msg);
                stream.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}