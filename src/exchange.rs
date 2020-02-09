use crate::connection::ConnectionFactory;
use crate::proto::stomp::STOMPParser;
use crate::queue_manager::QueueManager;

pub struct Exchange {
    //incoming: Connection, //TODO abhi: This should be multi-transport 
    name: String,
    parser: STOMPParser,
    queue_manager: QueueManager,
}

impl Exchange {
    pub fn new(name: String) -> Self {
        Exchange {
            //incoming: Connection::new("".parse().unwrap()),
            name: name,
            parser: STOMPParser,
            queue_manager: QueueManager::new()
        }
    }

    pub fn start(self) { 
        //self.incoming.start();
    }
}
