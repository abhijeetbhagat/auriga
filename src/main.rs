mod queue_manager;
mod broker;
mod connection;
mod exchange;
mod proto;

use broker::Broker;
use tokio;

fn main() {
    println!("Starting broker...");
    let broker = Broker::new();
    tokio::spawn(broker.start());
}
