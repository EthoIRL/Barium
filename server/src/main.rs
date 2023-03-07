use std::{
    sync::{Arc, Mutex}
};

use anticheat::node_manager::NodeManager;
use apis::plugin::server_manager;

pub mod anticheat {
    pub mod node_manager;

    pub mod node {
        pub mod node;
        pub mod player;
    }
}

pub mod apis {
    pub mod plugin {
        pub mod handlers {
            pub mod packet;
            pub mod service;
            pub mod play;

            pub mod packets {
                pub mod player_join_packet;
                pub mod player_leave_packet;
                pub mod node_register_packet;
                pub mod node_unregister_packet;
                
                pub mod player_movement_packet;
            }
        }
        pub mod server_manager;
    }
}

#[tokio::main]
async fn main() {
    let host = "127.0.0.1:8080";

    println!("Starting Barium Backend on ({})", host);

    let node_manager = NodeManager::default();
    let node_manager_arc = Arc::new(Mutex::new(node_manager));
    
    server_manager::init(node_manager_arc, host).await;
}