use std::error::Error;
use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = "127.0.0.1:61616";
    println!("Connecting to {} ...", server_addr);
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Consumer: Sending SUBSCRIBE to server ...");
    stream
        .write_all(b"SUBSCRIBE\nid:0\ndestination:/queue/foo\nack:client\n")
        .await?;
    let mut buf = [0; 100];
    loop {
        println!("Consumer: Waiting for message...");
        stream.read(&mut buf).await?;
        println!(
            "Consumer: Message recvd - {}",
            std::str::from_utf8(&buf).unwrap()
        );
    }
    Ok(())
}
