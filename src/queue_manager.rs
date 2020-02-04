use std::collections::VecDeque;
use crate::connection::Connection;

pub struct QueueManager {
    outgoing: Connection,
    queue: VecDeque<i32>
}

impl QueueManager {
    pub fn new() -> Self {
        QueueManager {
            outgoing: Connection::new("".parse().unwrap()),
            queue: VecDeque::new()
        }
    }
}
