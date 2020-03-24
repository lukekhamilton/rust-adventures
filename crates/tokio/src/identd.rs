use futures::sink::SinkExt;

use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};

use std::error::Error;
use tokio::stream::StreamExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let binding = ":::10113";
    let mut listener = TcpListener::bind(&binding).await?;
    println!("Server running at: {:?}", listener.local_addr().unwrap());

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let _ = handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let remote_ip = socket.peer_addr()?.ip();
    println!("Received a connection from {}", remote_ip);

    // version 1
    // loop {
    //     let mut buf = [0; 1024];
    //     let n = socket.read(&mut buf).await?;
    //
    //     if n == 0 {
    //         break;
    //     }
    //
    //     let received = String::from_utf8(buf[0..n].to_vec())?;
    //
    //     println!("They sent: {}", received);
    // }

    // version 2
    let mut client = Framed::new(socket, LinesCodec::new_with_max_length(1024));

    // Read one line of query
    // LinesCodec will accept either the required \r\n or a plain \n
    let query = match client.next().await {
        Some(Ok(q)) => q,
        _ => return Err("no query received".into()),
    };

    let (local_port, remote_port) = match parse_query(&query) {
        Ok((l, r)) => (l, r),
        Err(e) => {
            let response = format!("{} : ERROR : INVALID-PORT\r", query);
            println!("Error: {}", response);
            client.send(response).await?;
            return Err(e);
        }
    };

    println!("Local port: {}", local_port);
    println!("Remote port: {}", remote_port);

    Ok(())
}

#[derive(Debug)]
enum IdentError {
    NoQuery,
    InvalidPort,
}

fn parse_query(query: &str) -> Result<(u16, u16), Box<dyn Error + Send + Sync>> {
    let ports: Vec<&str> = query.split(",").map(|s| s.trim()).collect();

    if ports.len() != 2 {
        return Err("invalid format".into());
    }
    Ok((ports[0].parse()?, ports[1].parse()?))
}
