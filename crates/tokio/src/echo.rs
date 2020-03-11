use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
pub async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            match conn {
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr());

                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.split();

                        println!("{:?}", writer);
                        println!("{:?}", reader);

                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                eprintln!("wrote {} bytes", amt);
                            }
                            Err(err) => {
                                eprintln!("IO error {:?}", err);
                            }
                        }
                    });
                }
                Err(err) => {
                    eprintln!("Accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    server.await;
}
