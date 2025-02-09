use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::client::game::GameServer;
use crate::node::{auth, connection};

mod node;
mod packet;
mod proto;
mod client;

pub const API_VERSION: i32 = 0;

const NODE_GOVERNOR: (&str, u16) = ("127.0.0.1", 3349);

fn main() {
    println!("[NODE] Connecting to governor... {:?}", NODE_GOVERNOR);
    
    let mut stream = match TcpStream::connect(NODE_GOVERNOR) {
        Ok(stream) => stream,
        Err(err) => {
            println!("[NODE] Failed to connect to governor server... ({})", err);
            return;
        }
    };

    if let Err(err) = auth::authed_connection(&mut stream, "shared key") {
        println!("[NODE] Failed to send authed registration packet... ({})", err);
        return;
    }

    if let Err(err) = connection::handle_registration(&mut stream) {
        println!("[NODE] {}", err);
        return;
    };

    let servers: Arc<RwLock<HashMap<Uuid, Arc<GameServer>>>> = Arc::new(RwLock::new(HashMap::new()));

    connection::start_client_server(NODE_GOVERNOR, &mut stream, servers.clone());
}
