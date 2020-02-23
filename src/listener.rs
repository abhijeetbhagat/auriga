//use crate::client::Client;
use crate::message::Message;
use crate::proto::stomp::{Frame, STOMPCodec, STOMPFrame, STOMPParser};
//use crate::queue_manager::QueueManager;
use async_std::io::BufReader;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use futures::{channel::mpsc, select, FutureExt, SinkExt};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

type Tx<T> = mpsc::UnboundedSender<T>;
type Rx<T> = mpsc::UnboundedReceiver<T>;

pub struct ConnectionListener {
    addr: SocketAddr,
    //TODO abhi: use RwLock instead of a mutex?
    //Reason - subscribing will be a lesser activity than publishing
    //queue_manager: QueueManager,
}

impl ConnectionListener {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionListener {
            addr: addr,
            //queue_manager: QueueManager::new(), //connections_map: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn start(self) -> Result<(), io::Error> {
        println!("Starting server at {}", self.addr);
        let listener = TcpListener::bind(&self.addr).await?;
        let mut incoming = listener.incoming();
        //let queue_mgr = Arc::clone(&self.queue_manager);
        let (tx, rx) = mpsc::unbounded();
        let registrar = task::spawn(registrar(rx));
        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            println!("Accepted a connection from {}", stream.peer_addr()?);
            let tx = tx.clone();
            //let queue_mgr = Arc::clone(&queue_mgr);
            task::spawn(async move {
                /*if let Err(e) = self::handle(queue_mgr, stream, addr).await {
                    println!("Error handling the frame - {}", e);
                }*/
                if let Err(e) = self::socket_reader(tx, stream).await {
                    eprintln!("Error occurred in the socket_reader task {}", e);
                }
            });
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Void {}

#[derive(Debug)]
enum Event {
    NewPeer {
        name: SocketAddr,
        stream: Arc<TcpStream>,
        shutdown: Rx<Void>,
    },
    Message {
        from: SocketAddr,
        msg: String,
    },
}

async fn socket_reader(mut tx: Tx<STOMPFrame>, stream: TcpStream) -> Result<(), io::Error> {
    let mut reader = BufReader::new(stream);
    //let mut lines = reader.lines();
    let mut message = String::from("");
    let parser = STOMPParser;
    //while let Some(line) = lines.next().await {
    loop {
        let mut buf = Vec::with_capacity(2048);
        let size = reader.read_until(b'\0', &mut buf).await?;
        if size == 0 {
            break;
        }
        let message = String::from_utf8(buf).unwrap();
        let frame = parser.parse(&message);
        tx.send(frame).await;
    }
    println!("Done while ...");
    Ok(())
    //tx.send(stream.peer_addr().unwrap());
}

async fn registrar(mut rx: Rx<STOMPFrame>) {
    //let peers = HashMap::new();
    while let Some(frame) = rx.next().await {
        println!("{:?}", frame);
        match frame.r#type {
            Frame::Subscribe => {}
            Frame::Send => {}
            _ => {}
        }
    }
}

/*
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
            Err(e) => {
                println!("Error occurred {}", e);
                break;
                //return Err(Box::new(e));
            }
        }
    }
    //queue_mgr.lock().await.unsubscribe_disconnect(&addr);
    Ok(())
}
*/
