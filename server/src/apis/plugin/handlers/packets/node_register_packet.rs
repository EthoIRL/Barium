use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

use crate::anticheat::node_manager::NodeManager;

use crate::anticheat::node::node::{Node, Region};
use crate::apis::plugin::handlers::packet::to_buffer;

use rand::distributions::{Alphanumeric, DistString};
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeRegister {
    // pub barium_key: String,
    pub server_ip: String,
    pub server_version: String,
    pub server_region: Region,
    pub via_version: bool,
    pub bungee_cord: bool,
    pub cracked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeKey {
    pub key: String,
}

pub fn handle_node_register(packet_data: String, mut node_manager: MutexGuard<NodeManager>) -> Option<Vec<u8>> {
    let node_register: NodeRegister = match serde_json::from_str(&packet_data) {
        Ok(result) => result,
        Err(_) => return None,
    };

    let key = Alphanumeric.sample_string(&mut OsRng, 64);

    let node_key = NodeKey { key: key.clone() };

    let node = Node::new(
        node_register.server_ip.clone(),
        node_register.server_version.clone(),
        node_register.server_region.clone(),
        node_register.via_version.clone(),
        node_register.bungee_cord.clone(),
        node_register.cracked.clone(),
        key,
    );
    node_manager.add_node(node);

    println!("DATA: {:#?}", node_register);
    println!("KEY: {:#?}", node_key);

    for node in &node_manager.nodes {
        println!("NODE KEY: {:#?}", node.key);
        println!("NODE VERSION: {:#?}", node.server_version);
    }

    let packet = to_buffer::<NodeKey>([0], node_key);
    return Some(packet);
}