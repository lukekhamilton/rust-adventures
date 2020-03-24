use std::error::Error;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::delay_for;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "::1:10113";
    let mut socket = TcpStream::connect(addr).await?;

    delay_for(Duration::from_millis(1000)).await;

    println!("Chunk 1");
    socket.write_all("37950".as_bytes()).await?;

    delay_for(Duration::from_millis(500)).await;

    println!("Chunk 2");
    socket.write_all(",22".as_bytes()).await?;

    delay_for(Duration::from_millis(500)).await;

    println!("Chunk 3");
    socket.write_all("\r\n".as_bytes()).await?;

    Ok(())
}
