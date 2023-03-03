use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

use crate::anticheat::node_manager::NodeManager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeUnregister {
    pub key: String
}

pub fn handle_node_unregister(packet_data: String, mut node_manager: MutexGuard<NodeManager>) {
    let node_unregister: NodeUnregister = match serde_json::from_str(&packet_data) {
        Ok(result) => result,
        Err(_) => return
    };

    println!("Removing registered node from manager: {:#?}", node_unregister);

    node_manager.remove_node(node_unregister.key);
}