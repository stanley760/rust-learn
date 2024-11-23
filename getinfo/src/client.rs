use core::panic;
use std::env;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args()
        .nth(1)
        .unwrap_or_else(||"127.0.0.1:8088".to_string());
    let stream = TcpStream::connect(&args).await?;

    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    framed.send(Bytes::from("gettime")).await?;

    if let Some(msg) = framed.next().await {
       match msg {
           Ok(msg) => {
               let timeinfo = String::from_utf8(msg.to_vec());
               println!("{}", timeinfo.unwrap());
           },
           Err(e) => panic!("{}", e),
       }
    }

    

    Ok(())
}
