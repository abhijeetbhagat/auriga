use crate::exchange::Exchange;
use std::thread;

pub struct Broker {
    exchange: Exchange,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            exchange: Exchange::new(String::from("")),
        }
    }

    pub async fn start(self) {
        //TODO abhi: start protocol handler
        self.exchange.start().await;
    }

    fn route_message(self, key: &str) {}
}
