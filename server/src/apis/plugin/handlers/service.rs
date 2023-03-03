use std::{
    i8,
    sync::{Arc, Mutex},
};

use crate::anticheat::node_manager::NodeManager;
use crate::apis::plugin::handlers::packet::Packet;
use crate::apis::plugin::handlers::packet::to_string;

use super::packets::node_register_packet;
use super::packets::node_unregister_packet;

pub fn handle_service_packet(
    packet: Packet,
    node_manager_arc: &Arc<Mutex<NodeManager>>,
) -> Option<Vec<u8>> {
    let id = packet.id;
    let packet_data: String = match to_string(packet) {
        Some(data) => data,
        None => {
            panic!("No string data found in packet!")
        }
    };

    let node_manager = match node_manager_arc.lock() {
        Ok(result) => result,
        Err(_) => return None,
    };

    match id as i8 {
        0 => {
            return node_register_packet::handle_node_register(packet_data, node_manager);
        },
        1 => {
            node_unregister_packet::handle_node_unregister(packet_data, node_manager);
        }
        _ => todo!(),
    }

    None
}
