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
use crate::queue_manager::QueueManager;
use crate::proto::stomp::{ Frame, STOMPParser };

type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

pub struct ConnectionListener {
    addr: SocketAddr,
    queue_manager: Arc<Mutex<QueueManager>>,
    //connections_map: Arc<Mutex<HashMap<SocketAddr, Tx>>>
}

enum Message {
    StreamMessage(String), 
    ChannelMessage(String)
}

struct Client {
    rx: Option<Rx>,
    stream: Framed<TcpStream, LinesCodec>
}

impl ConnectionListener {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionListener {
            addr: addr,
            queue_manager: Arc::new(Mutex::new(QueueManager::new()))
            //connections_map: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn start(self) {
        let server = async move {
            println!("Starting server at {}", self.addr);
            let mut listener = TcpListener::bind(&self.addr).await.unwrap();
            let queue_mgr = Arc::clone(&self.queue_manager);
            loop {
                let (stream, addr) = listener.accept().await.unwrap();
                println!("Accepted a connection from {}", addr);
                let queue_mgr = Arc::clone(&queue_mgr);
                tokio::spawn(async move {
                    self::create_client(queue_mgr, stream, addr).await;
                });
            }
        };
        server.await;
    } 
}

impl Stream for Client { 
    type Item = Result<Message, LinesCodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(v)) = Pin::new(self.rx.as_mut().unwrap()).poll_next(cx) {
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

async fn create_client(queue_mgr: Arc<Mutex<QueueManager>>, stream: TcpStream, addr: SocketAddr) { 
    let mut stream = Framed::new(stream, LinesCodec::new());

    let mut client = Client { rx: None, stream: stream };
    let parser = STOMPParser;
    while let Some(result) = client.next().await { 
        match result {
            Ok(Message::ChannelMessage(m)) => {
                client.stream.send(m).await;
            },
            Ok(Message::StreamMessage(m)) => {
                let msg = &m;
                let frame = parser.parse(msg);
                match frame {
                    Frame::Subscribe => { 
                        let (tx, rx) = mpsc::unbounded_channel();
                        client.rx = Some(rx);
                        queue_mgr.lock().await.subscribe(String::from(""), tx, addr);
                    },
                    Frame::Unsubscribe => { 
                        queue_mgr.lock().await.unsubscribe();
                    },
                    Frame::Send => { 
                        queue_mgr.lock().await.publish("", String::from(msg), &addr);
                    }
                    _ => { }
                }
            },
            Err(e) => {}
        }
    }
}