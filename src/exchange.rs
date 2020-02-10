use crate::connection::ConnectionFactory;
use crate::proto::stomp::STOMPParser;
use crate::queue_manager::QueueManager;

pub struct Exchange {
    //incoming: Connection, //TODO abhi: This should be multi-transport 
    name: String,
    parser: STOMPParser,
    queue_manager: QueueManager,
    connection_factory: ConnectionFactory
}

impl Exchange {
    pub fn new(name: String) -> Self {
        Exchange {
            //incoming: Connection::new("".parse().unwrap()),
            name: name,
            parser: STOMPParser,
            queue_manager: QueueManager::new(),
            connection_factory: ConnectionFactory::new("127.0.0.1:61616".parse().unwrap())
        }
    }

    pub async fn start(self) { 
        self.connection_factory.start().await;
    }
}
