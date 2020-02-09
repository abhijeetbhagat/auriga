use std::thread;
use crate::exchange::Exchange;

pub struct Broker{
    exchange: Exchange,
}

impl Broker {
    pub fn new() -> Self { 
        Broker {
            exchange: Exchange::new(String::from(""))
        }
    }

    pub async fn start(self) { 
        //TODO abhi: start protocol handler
        self.exchange.start();
    }

    fn route_message(self, key: &str) {

    }

}
