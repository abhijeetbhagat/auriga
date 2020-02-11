use tokio::net::{TcpListener, TcpStream};
use futures::SinkExt; 
use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use tokio::stream::{Stream, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::io;
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};


type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

pub struct ConnectionListener {
    addr: SocketAddr,
    connections_map: Arc<Mutex<HashMap<SocketAddr, Tx>>>
}

enum Message {
    StreamMessage(String), 
    ChannelMessage(String)
}

struct Client {
    rx: Rx,
    stream: Framed<TcpStream, LinesCodec>
}

impl ConnectionListener {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionListener {
            addr: addr,
            connections_map: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn start(self) {
        let server = async move {
            println!("Starting server at {}", self.addr);
            let mut listener = TcpListener::bind(&self.addr).await.unwrap();
            let map = Arc::clone(&self.connections_map);
            loop {
                let (stream, addr) = listener.accept().await.unwrap();
                println!("Accepted a connection from {}", addr);
                let map = Arc::clone(&map);
                tokio::spawn(async move {
                    self::create_client(map, stream, addr).await;
                });
            }
        };
        server.await;
    } 
}

impl Stream for Client { 
    type Item = Result<Message, LinesCodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(v)) = Pin::new(&mut self.rx).poll_next(cx) {
            return Poll::Ready(Some(Ok(Message::ChannelMessage(v))))
        } 

        let result: Option<_> = futures::ready!(Pin::new(&mut self.stream).poll_next(cx));
        Poll::Ready(match result { 
            Some(Ok(message)) => Some(Ok(Message::StreamMessage(message))),
            Some(Err(e)) => Some(Err(e)),
            None => None
        })
    }
}

async fn create_client(connections_map: Arc<Mutex<HashMap<SocketAddr, Tx>>>, stream: TcpStream, addr: SocketAddr) { 
    let (tx, rx) = mpsc::unbounded_channel();
    {
        connections_map.lock().await.insert(addr, tx); //get a write lock
        //map.insert(addr, tx);
    }
    let mut lines = Framed::new(stream, LinesCodec::new());
    println!("Gon send msg to connected client ...");
    lines.send(String::from("Enter your name")).await;
    let name = match lines.next().await { 
        Some(Ok(line)) => line,
        _ => {
            println!("Failed to get name"); 
            return;
        }
    };

    println!("server: {} has joined", name);

    {
        for p in connections_map.lock().await.iter_mut() { 
            if *p.0 != addr {
                let _ = p.1.send(format!("{} has joined", name));
            }
        }
    }

    let mut client = Client { rx: rx, stream: lines };
    while let Some(result) = client.next().await { 
        match result {
            Ok(Message::ChannelMessage(m)) => {
                client.stream.send(m).await;
            },
            Ok(Message::StreamMessage(m)) => {
                let msg = &m;
                for p in connections_map.lock().await.iter_mut() {
                    if *p.0 != addr {
                        let _ = p.1.send(msg.into());
                    }
                }
            },
            Err(e) => {}
        }
    }
}