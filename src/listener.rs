use crate::client::Client;
use crate::message::Message;
use crate::proto::stomp::{Frame, STOMPCodec};
use crate::queue_manager::QueueManager;
use futures::SinkExt;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::Framed;

pub struct ConnectionListener {
    addr: SocketAddr,
    //TODO abhi: use RwLock instead of a mutex?
    //Reason - subscribing will be a lesser activity than publishing
    queue_manager: Arc<Mutex<QueueManager>>,
}

impl ConnectionListener {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionListener {
            addr: addr,
            queue_manager: Arc::new(Mutex::new(QueueManager::new())), //connections_map: Arc::new(Mutex::new(HashMap::new()))
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

async fn handle(
    queue_mgr: Arc<Mutex<QueueManager>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let stream = Framed::new(stream, STOMPCodec::new());
    let mut client = Client {
        rx: None,
        stream: stream,
    };
    //let parser = STOMPParser;

    println!("Starting handler for {}", addr);
    while let Some(result) = client.next().await {
        match result {
            Ok(Message::ChannelMessage(m)) => {
                println!("Got something from channel to send on the write part of the stream");
                client.stream.send(m).await?;
            }
            Ok(Message::StreamMessage(m)) => {
                println!("Got something to process from the read part of the stream");
                let frame = &m;
                println!("frame received: {}", frame);
                match frame.r#type {
                    Frame::Subscribe => {
                        if !queue_mgr.lock().await.query_subscription(frame, &addr) {
                            let (tx, rx) = mpsc::unbounded_channel();
                            client.rx = Some(rx);
                            queue_mgr
                                .lock()
                                .await
                                .subscribe(frame, tx, addr, &mut client);
                        }
                    }
                    Frame::Unsubscribe => {
                        queue_mgr.lock().await.unsubscribe(frame, &addr);
                    }
                    Frame::Send => {
                        let routing_key = frame.headers.get("destination").unwrap();
                        queue_mgr.lock().await.publish(routing_key, frame, &addr);
                    }
                    _ => {}
                }
            }
            Err(e) => {}
        }
    }
    Ok(())
}
