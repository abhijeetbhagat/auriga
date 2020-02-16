use crate::listener::ConnectionListener;
use crate::proto::stomp::STOMPParser;
use crate::queue_manager::QueueManager;

pub struct Exchange {
    name: String,
    parser: STOMPParser,
    queue_manager: QueueManager,
    connection_factory: ConnectionListener,
}

impl Exchange {
    pub fn new(name: String) -> Self {
        Exchange {
            name: name,
            parser: STOMPParser,
            queue_manager: QueueManager::new(),
            connection_factory: ConnectionListener::new("127.0.0.1:61616".parse().unwrap()),
        }
    }

    pub async fn start(self) {
        self.connection_factory.start().await;
    }
}
