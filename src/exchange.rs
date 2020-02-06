use crate::connection::Connection;
use crate::proto::stomp::STOMPParser;
use crate::queue_manager::QueueManager;

pub struct Exchange {
    //incoming: Connection, //TODO abhi: This should be multi-transport 
    parser: STOMPParser,
    queue_manager: QueueManager,
}

impl Exchange {
    pub fn new() -> Self {
        Exchange {
            //incoming: Connection::new("".parse().unwrap()),
            parser: STOMPParser,
            queue_manager: QueueManager::new()
        }
    }

    pub fn start(self) { 
        //self.incoming.start();
    }
}
