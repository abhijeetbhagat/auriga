/*use crate::client::Client;
use crate::proto::stomp::STOMPFrame;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::SocketAddr;
use tokio::sync::mpsc;

type Tx = mpsc::UnboundedSender<STOMPFrame>;

struct Subscriber {
    tx: Tx,
    addr: SocketAddr,
}

struct Queue {
    queue: VecDeque<STOMPFrame>,
    subscribers: Vec<Subscriber>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
            subscribers: Vec::new(),
        }
    }
}

pub struct QueueManager {
    queue_map: HashMap<String, Queue>,
}

impl QueueManager {
    pub fn new() -> Self {
        QueueManager {
            queue_map: HashMap::new(),
        }
    }

    pub fn create_queue(&mut self, routing_key: &str) {
        if !self.queue_map.contains_key(routing_key) {
            let queue = Queue::new();
            self.queue_map.insert(String::from(routing_key), queue);
        }
    }

    //TODO abhi: for STOMP, if the subscription fails, then we should
    //send an ERROR frame to the client. But we are always making the
    //subscription succeed here.
    pub fn subscribe(&mut self, frame: &STOMPFrame, tx: Tx, addr: SocketAddr, client: &mut Client) {
        let routing_key = frame.headers.get("destination").unwrap();
        if !self.queue_map.contains_key(routing_key) {
            let mut queue = Queue::new();
            queue.subscribers.push(Subscriber { addr: addr, tx: tx });
            println!("inserted {} routing key", routing_key);
            self.queue_map.insert(String::from(routing_key), queue);
        } else {
            let queue = self.queue_map.get_mut(routing_key).unwrap();
            queue.subscribers.push(Subscriber { addr: addr, tx: tx });
        }
    }

    //TODO abhi: this does linear searching.
    //should use something other than a list to store subscribers?
    pub fn query_subscription(&self, frame: &STOMPFrame, addr: &SocketAddr) -> bool {
        let routing_key = frame.headers.get("destination").unwrap();
        if self.queue_map.contains_key(routing_key) {
            let queue = self.queue_map.get(routing_key).unwrap();
            return queue
                .subscribers
                .iter()
                .find(|subscriber| subscriber.addr == *addr)
                .is_some();
        }
        false
    }

    pub fn unsubscribe(&mut self, frame: &STOMPFrame, addr: &SocketAddr) {
        let routing_key = frame.headers.get("destination").unwrap();
        if self.query_subscription(frame, addr) {
            let queue = self.queue_map.get_mut(routing_key).unwrap();
            let pos = queue
                .subscribers
                .iter()
                .position(|subscriber| subscriber.addr == *addr)
                .unwrap();
            let subscriber = queue.subscribers.remove(pos);
            println!("Removed subscriber - {}", subscriber.addr);
        }
    }

    pub fn publish(&mut self, routing_key: &str, msg: &STOMPFrame, sender: &SocketAddr) {
        match self.queue_map.get_mut(routing_key) {
            Some(q) => {
                q.queue.push_back(msg.clone());
                let mut failure_subscribers = vec![];
                for subscriber in q.subscribers.iter() {
                    if subscriber.addr != *sender {
                        println!("Sending message to {}", subscriber.addr);
                        match subscriber.tx.send(msg.clone()) {
                            Err(e) => {
                                //TODO: unsubscribe the recvr socket
                                println!("recv end seemed to have closed");
                                failure_subscribers.push(subscriber.addr);
                            }
                            _ => {}
                        }
                    }
                }
                for addr in failure_subscribers.iter() {
                    self.unsubscribe(msg, addr);
                }
            }
            None => {
                println!("No queue associated with routing key {} found", routing_key);
            }
        }
    }

    pub fn unsubscribe_disconnect(&mut self, addr: &SocketAddr) {}
}
*/
