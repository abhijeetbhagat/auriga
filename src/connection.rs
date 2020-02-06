use tokio::net::TcpListener;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use std::net::SocketAddr;
use tokio::stream::StreamExt;

pub struct Connection<'a>{
    reader: ReadHalf<'a>,
    writer: WriteHalf<'a>
}

pub struct ConnectionFactory {
    addr: SocketAddr
}

impl ConnectionFactory {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionFactory {
            addr: addr
        }
    }

    pub async fn start(self) {
        let server = async move {
            let mut listener = TcpListener::bind(&self.addr).await.unwrap();

            let mut incoming = listener.incoming();
            while let Some(socket_res) = incoming.next().await {
                match socket_res { 
                    Ok(mut socket) => {
                        println!("Accepted connection from {:?}", socket.peer_addr());
                        let (mut reader, mut writer) = socket.split();
                        let c = Connection {
                            reader: reader,
                            writer: writer
                        };
                    },
                    Err(err) => { 
                        println!("Error accepting = {:?}", err);
                    }
                } 
            }
        };
        server.await;
    }
}