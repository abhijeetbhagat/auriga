use std::error::Error;
use std::{thread, time};
use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = "127.0.0.1:61616";
    println!("Connecting to {} ...", server_addr);
    let mut stream = TcpStream::connect(server_addr).await?;
    //stream.write_all(b"CONNECT\naccept-version:1.0,1.1,2.0\nhost:auriga1.universe.com\n").await?;
    println!("Producer: Sending SUBSCRIBE to server ...");
    let res = stream
        .write(b"SUBSCRIBE\nid:0\ndestination:/queue/foo\nack:client\n")
        .await;
    println!("SUBSCRIBE write result is {}", res.is_ok());
    println!("Producer: Sending SEND to server ...");
    loop {
        let res = stream
            .write(
                b"SEND\ndestination:/queue/foo\ncontent-type:text/plain\ncontent-length:5\nabhi\n",
            )
            .await;
        println!("SEND write result is {}", res.is_ok());
        if !res.is_ok() {
            break;
        }
        let two_secs = time::Duration::from_secs(2);
        thread::sleep(two_secs);
    }
    Ok(())
}
