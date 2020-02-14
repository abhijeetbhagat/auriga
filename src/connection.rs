use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use tokio::stream::{ Stream, StreamExt};
use std::sync::Arc;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::io;
use std::error::Error;
use tokio_util::codec::{Framed};
use crate::queue_manager::QueueManager;
use crate::proto::stomp::{ STOMPCodec, Frame, STOMPFrame };

type Rx = mpsc::UnboundedReceiver<STOMPFrame>;

pub struct ConnectionListener {
    addr: SocketAddr,
    //TODO abhi: use RwLock instead of a mutex?
    //Reason - subscribing will be a lesser activity than publishing
    queue_manager: Arc<Mutex<QueueManager>>,
}

enum Message {
    StreamMessage(STOMPFrame), 
    ChannelMessage(STOMPFrame)
}

struct Client {
    rx: Option<Rx>,
    stream: Framed<TcpStream, STOMPCodec>
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
                    if let Err(e) = self::handle(queue_mgr, stream, addr).await {
                        println!("Error handling the frame - {}", e);
                    }
                });
            }
        };
        server.await;
    } 
}

impl Stream for Client { 
    type Item = Result<Message, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_some() { //we initialize rx only when a socket subscribes to a queue
            if let Poll::Ready(Some(v)) = Pin::new(self.rx.as_mut().unwrap()).poll_next(cx) {
                return Poll::Ready(Some(Ok(Message::ChannelMessage(v))))
            } 
        }

        let result: Option<_> = futures::ready!(Pin::new(&mut self.stream).poll_next(cx));
        Poll::Ready(match result { 
            Some(Ok(message)) => { 
                println!("Poller: message recvd - {}", message);
                Some(Ok(Message::StreamMessage(message)))
            },
            Some(Err(e)) => Some(Err(e)),
            None => None
        })
    }
}

async fn handle(
                queue_mgr: Arc<Mutex<QueueManager>>, 
                stream: TcpStream, 
                addr: SocketAddr
               ) -> Result<(), Box<dyn Error>> { 
    let stream = Framed::new(stream, STOMPCodec::new());
    let mut client = Client { rx: None, stream: stream };
    //let parser = STOMPParser;

    println!("Starting handler for {}", addr);
    while let Some(result) = client.next().await { 
        match result {
            Ok(Message::ChannelMessage(m)) => {
                println!("Got something from channel to send on the write part of the stream");
                //client.stream.send(m).await;
            }
            Ok(Message::StreamMessage(m)) => {
                println!("Got something to process from the read part of the stream");
                let frame = &m;
                println!("frame received: {}", frame);
                match frame.r#type {
                    Frame::Subscribe => { 
                        let routing_key = frame.headers.get("destination").unwrap();
                        if !queue_mgr.lock().await.query_subscription(&routing_key, &addr) { 
                            let (tx, rx) = mpsc::unbounded_channel();
                            client.rx = Some(rx);
                            queue_mgr.lock().await.subscribe(routing_key, tx, addr);
                        }
                    }
                    Frame::Unsubscribe => { 
                        queue_mgr.lock().await.unsubscribe();
                    }
                    Frame::Send => { 
                        let routing_key = frame.headers.get("destination").unwrap();
                        queue_mgr.lock().await.publish(routing_key, frame, &addr);
                    }
                    _ => { }
                }
            }
            Err(e) => {}
        }
    }
    Ok(())
}