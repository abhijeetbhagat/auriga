use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use std::{thread, time};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(try_main())
}

async fn try_main() -> Result<()> {
    let server_addr = "127.0.0.1:61616";
    println!("Connecting to {} ...", server_addr);
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Local addr is {:?}", stream.local_addr()?);
    //stream.write_all(b"CONNECT\naccept-version:1.0,1.1,2.0\nhost:auriga1.universe.com\n").await?;
    println!("Producer: Sending SUBSCRIBE to server ...");
    let res = stream
        .write(b"SUBSCRIBE\nid:0\ndestination:/queue/foo\nack:client\n\0")
        .await;
    println!("SUBSCRIBE write result is {}", res.is_ok());
    //loop {
    println!("Producer: Sending SEND to server ...");
    let res = stream
        .write(b"SEND\ndestination:/queue/foo\ncontent-type:text/plain\ncontent-length:5\nabhi\n\0")
        .await;
    println!("SEND write result is {}", res.is_ok());
    /*    let two_secs = time::Duration::from_secs(2);
        thread::sleep(two_secs);
    }*/
    Ok(())
}
