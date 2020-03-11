use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
pub async fn hello() {
    println!("Hello world");
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("Created stream: {:?}", stream);

    let result = stream.write(b"Hello world\n").await;
    println!("wrote to stream; success={:#?}", result.is_ok());
}

#[test]
fn test_main() {
    hello();
}
