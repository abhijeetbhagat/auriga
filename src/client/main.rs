use tokio::net::TcpStream;
use tokio::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> { 
    let server_addr = "127.0.0.1:61616";
    println!("Connecting to {} ...", server_addr);
    let mut stream = TcpStream::connect(server_addr).await?; 
    println!("Sending SUBSCRIBE to server ...");
    stream.write_all(b"SUBSCRIBE").await?;
    let mut buf = [0; 50];
    stream.read(&mut buf).await?;
    Ok(())
}