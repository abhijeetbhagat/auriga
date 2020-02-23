use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(try_main())
}

async fn try_main() -> Result<()> {
    let server_addr = "127.0.0.1:61616";
    println!("Connecting to {} ...", server_addr);
    let mut stream = TcpStream::connect(server_addr).await?;
    let (mut reader, mut writer) = (&stream, &stream);
    println!("Consumer: Sending SUBSCRIBE to server ...");
    writer
        .write_all(b"SUBSCRIBE\nid:0\ndestination:/queue/foo\nack:client\n\0")
        .await?;
    let mut buf = [0; 100];
    loop {
        println!("Consumer: Waiting for message...");
        reader.read(&mut buf).await?;
        println!(
            "Consumer: Message recvd - {}",
            std::str::from_utf8(&buf).unwrap()
        );
    }
    Ok(())
}
