use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use crate::server::{client, node};
use crate::server::node::Node;

mod server;
mod proto;
mod plugin;
pub mod packet;
mod anticheat;
mod error;

pub const API_VERSION: i32 = 0;
const CLIENT_ADDRESS: (&str, u16) = ("127.0.0.1", 3238);
const NODE_ADDRESS: (&str, u16) = ("127.0.0.1", 3349);

const NODE_KEY: &str = "shared key";

fn main() {
    println!("[GOV] Starting proxy server, {:?}", &CLIENT_ADDRESS);
    println!("[GOV] Starting node server, {:?}", &NODE_ADDRESS);

    let nodes: Arc<RwLock<HashMap<Uuid, Arc<Node>>>> = Arc::new(RwLock::new(HashMap::new()));

    if let Err(err) = node::start_node_server(NODE_ADDRESS, nodes.clone()) {
        eprintln!("[GOV] Failed to start node server, ({})", err);
        return;
    }

    if let Err(err) = client::start_plugin_server(CLIENT_ADDRESS, nodes.clone()) {
        eprintln!("[GOV] Failed to start client server, ({})", err);
        return;
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
