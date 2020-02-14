use tokio::net::{TcpListener, TcpStream};
use futures::SinkExt; 
use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use tokio::stream::{ Stream, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::io;
use std::error::Error;
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};
use crate::queue_manager::QueueManager;
use crate::proto::stomp::{ STOMPCodec, Frame, STOMPFrame };
use crate::proto::custom::{ Custom};
use tokio_util::codec::{BytesCodec, Decoder};

type Tx = mpsc::UnboundedSender<STOMPFrame>;
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
    stream: Framed<TcpStream, STOMPFrame>
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
                //let queue_mgr = Arc::clone(&queue_mgr);
                tokio::spawn(async move {
                    if let Err(e) = self::handle(stream, addr).await {
                        println!("Error handling the frame - {}", e);
                    }
                });
            }
        };
        server.await;
    } 
}

/*impl Stream for Client { 
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
}*/

async fn handle(
                //queue_mgr: Arc<Mutex<QueueManager>>, 
                stream: TcpStream, 
                addr: SocketAddr
               ) -> Result<(), Box<dyn Error>> { 
    let mut stream = Framed::new(stream, STOMPCodec::new());
    //let mut client = Client { rx: None, stream: stream };
    //let parser = STOMPParser;

    println!("Starting handler for {}", addr);
    //while let Some(result) = client.next().await { 
    while let Some(result) = stream.next().await { 
        //println!("{}", result);
        match result {
            Ok(frame) => {
                println!("bytes received - {:?}", frame);
                //stream.send(frame.freeze()).await?;
            }
            Err(e) => return Err(e.into())
        }
        /*match result.r#type {
            Ok(Message::ChannelMessage(m)) => {
                println!("Got something from channel to send on the write part of the stream");
                //client.stream.send(m).await;
            },
            Ok(Message::StreamMessage(m)) => {
                println!("Got something to process from the read part of the stream");
                let frame = &m;
                println!("frame received: {}", frame);
                //let frame = parser.parse(msg);
                match frame.r#type {
                    Frame::Subscribe => { 
                        let (tx, rx) = mpsc::unbounded_channel();
                        //client.rx = Some(rx);
                        let routing_key = frame.headers["destination"].clone();
                        queue_mgr.lock().await.subscribe(routing_key, tx, addr);
                    },
                    Frame::Unsubscribe => { 
                        queue_mgr.lock().await.unsubscribe();
                    },
                    Frame::Send => { 
                        queue_mgr.lock().await.publish("", frame, &addr);
                    }
                    _ => { }
                }
            },
            Err(e) => {}
        }*/
    }
    Ok(())
}