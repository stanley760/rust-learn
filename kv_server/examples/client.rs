use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_server::{CommandRequest, CommandResponse};
use std::result::Result::Ok;
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // define the address.
    let addr = "localhost:9527";
    // connect server.
    let stream = TcpStream::connect(addr).await?;
    // handle frame using AsyncProstStream.
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // create with the command named HSET.
    let cmd = CommandRequest::new_hset("table1", "hello", "rust".into());
    // send the command HSET.
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    Ok(())
}
