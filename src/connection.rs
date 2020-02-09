use tokio::net::{TcpListener, TcpStream};
use futures::SinkExt; 
use std::net::SocketAddr;
use tokio::sync::{mpsc, RwLock};
use tokio::stream::{Stream, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::io;
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};


type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

pub struct ConnectionFactory {
    addr: SocketAddr,
    connections_map: Arc<RwLock<HashMap<SocketAddr, Tx>>>
}

enum Message {
    StreamMessage(String), 
    ChannelMessage(String)
}

struct Client {
    rx: Rx,
    stream: Framed<TcpStream, LinesCodec>
}

impl ConnectionFactory {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionFactory {
            addr: addr,
            connections_map: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn start(self) {
        let server = async move {
            let mut listener = TcpListener::bind(&self.addr).await.unwrap();
            let map = Arc::clone(&self.connections_map);
            loop {
                let (stream, addr) = listener.accept().await.unwrap();
                let map = Arc::clone(&map);
                tokio::spawn(async move {
                    self::create_client(map, stream, addr);
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

async fn create_client(connections_map: Arc<RwLock<HashMap<SocketAddr, Tx>>>, stream: TcpStream, addr: SocketAddr) { 
    let (tx, rx) = mpsc::unbounded_channel();
    let mut map = connections_map.write().await; //get a write lock
    map.insert(addr, tx);
    let mut client = Client { rx: rx, stream: Framed::new(stream, LinesCodec::new()) };
    while let Some(result) = client.next().await { 
        match result {
            Ok(Message::ChannelMessage(m)) => {},
            Ok(Message::StreamMessage(m)) => {},
            Err(e) => {}
        }
    }
}