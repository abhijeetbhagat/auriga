use std::collections::VecDeque;
use std::collections::HashMap;
use tokio::sync::{mpsc};
use std::net::SocketAddr;
//use crate::connection::Connection;

type Tx = mpsc::UnboundedSender<String>;

struct Subscriber {
    tx: Tx,
    addr: SocketAddr
}

struct Queue {
    queue: VecDeque<String>,
    subscribers: Vec<Subscriber>
}

impl Queue {
    fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
            subscribers: Vec::new()
        }
    }
}

pub struct QueueManager {
    queue_map: HashMap<String, Queue> 
}

impl QueueManager {
    pub fn new() -> Self {
        QueueManager {
            queue_map: HashMap::new()
        }
    }

    pub fn subscribe(&mut self, routing_key: String, tx: Tx, addr: SocketAddr) {
        if !self.queue_map.contains_key(&routing_key) {
            let mut queue = Queue::new();
            queue.subscribers.push(Subscriber{addr: addr, tx: tx});
            println!("inserted {} routing key", routing_key);
            self.queue_map.insert(routing_key, queue);
        }
    }

    pub fn unsubscribe(&mut self) {

    }

    pub fn publish(&mut self, routing_key: &str, msg: String, addr: &SocketAddr) {
        match self.queue_map.get_mut(routing_key) { 
            Some(q) => { 
                q.queue.push_back(msg.clone());
                for subscriber in q.subscribers.iter() {
                    if subscriber.addr != *addr { 
                        subscriber.tx.send(msg.clone());
                    }
                }
            }
            None => {
                println!("No queue associated with routing key {} found", routing_key);
            }
        }
    }
}
