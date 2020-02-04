use tokio::net::TcpListener;
use std::net::SocketAddr;
use tokio::stream::StreamExt;

pub struct Connection{
    addr: SocketAddr
}

impl Connection {
    pub fn new(addr: SocketAddr) -> Self {
        Connection {
            addr: addr
        }
    }

    pub async fn start(self) { 
        let server = async move {
            let mut listener = TcpListener::bind(&self.addr).await.unwrap();

            let mut incoming = listener.incoming();
            while let Some(socket_res) = incoming.next().await {
                match socket_res { 
                    Ok(socket) => {
                        println!("Accepted connection from {:?}", socket.peer_addr());
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
