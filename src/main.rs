mod queue_manager;
mod broker;
mod connection;
mod exchange;
mod proto;

use broker::Broker;
use tokio;

#[tokio::main]
async fn main() {
    println!("Starting broker...");
    let broker = Broker::new();
    // tokio::spawn(broker.start());
    broker.start().await;
}
