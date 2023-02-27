use std::{
    sync::{Arc, Mutex}
};

use anticheat::node_manager::{node_manager::NodeManager, self};

use crate::apis::plugin::server_manager::server_manager;

pub mod anticheat {
    pub mod node_manager;
}

pub mod apis {
    pub mod frontend {
        // TODO
    }
    pub mod plugin {
        pub mod packets {
            pub mod packet;
            pub mod service;
        }
        pub mod server_manager;
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let node_manager = NodeManager::default();
    let node_manager_arc = Arc::new(Mutex::new(node_manager));
    
    server_manager::init(node_manager_arc).await;
}