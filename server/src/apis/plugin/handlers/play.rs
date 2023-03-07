use std::sync::{Arc, Mutex};

use crate::anticheat::node_manager::NodeManager;
use crate::apis::plugin::handlers::packet::Packet;

use crate::apis::plugin::handlers::packet::to_string;

use super::packets::player_join_packet;
use super::packets::player_leave_packet;
use super::packets::player_movement_packet;

pub fn handle_play_packet(
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
        10 => {
            player_join_packet::handle_player_join(packet_data, node_manager);
        },
        11 => {
            player_leave_packet::handle_player_leave(packet_data, node_manager);
        },
        12 => {
            player_movement_packet::handle_player_movement(packet_data, node_manager);
        },
        _ => todo!(),
    }

    None
}
