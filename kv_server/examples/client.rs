use anyhow::{Ok, Result};
use async_prost::AsyncProstStream;
use kv_server::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;
use futures::prelude::*;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // define the address.
    let addr = "127.0.0.1:9527";
    // connect server.
    let stream = TcpStream::connect(addr).await?;
    // handle frame using AsyncProstStream.
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // create with the command named HSET.
    let cmd = CommandRequest::new_hset("table1", "hello", "sally".into());
    // send the command HSET. 
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    Ok(())
}