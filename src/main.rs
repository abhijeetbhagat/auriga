mod broker;
mod client;
mod exchange;
mod listener;
mod message;
mod proto;
mod queue_manager;

use broker::Broker;
use tokio;

#[tokio::main]
async fn main() {
    println!("Starting broker...");
    let broker = Broker::new();
    // tokio::spawn(broker.start());
    broker.start().await;
}
