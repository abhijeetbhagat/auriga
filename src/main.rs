mod broker;
mod client;
mod exchange;
mod listener;
mod message;
mod proto;
mod queue_manager;

use async_std::task;
use broker::Broker;

fn main() {
    println!("Starting broker...");
    let broker = Broker::new();
    // tokio::spawn(broker.start());
    task::block_on(broker.start());
}
