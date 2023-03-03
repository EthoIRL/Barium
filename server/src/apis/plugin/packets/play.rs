use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::anticheat::node_manager::NodeManager;
use crate::{anticheat::node::player::NodePlayer, apis::plugin::packets::packet::Packet};

use crate::anticheat::node::node::{Node, Region};
use crate::apis::plugin::packets::packet::{to_buffer, to_string};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerJoin {
    pub server_key: String,
    pub username: String,
    pub uuid: Option<String>,
    pub version: Option<String>,
}

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

    let mut node_manager = match node_manager_arc.lock() {
        Ok(result) => result,
        Err(_) => return None,
    };

    match id as i8 {
        10 => {
            let player_join: PlayerJoin = match serde_json::from_str(&packet_data) {
                Ok(result) => result,
                Err(_) => return None,
            };

            match node_manager.get_node(player_join.server_key.clone()) {
                Some(node) => {
                    let node_player = NodePlayer {
                        username: player_join.username,
                        uuid: player_join.uuid,
                        version: player_join.version,
                        player_data: None,
                    };

                    println!("PLAYER JOINED: {:#?}", node_player);

                    node.add_player(node_player);
                }
                None => {
                    eprintln!(
                        "Failed to find node from server key {}",
                        player_join.server_key
                    )
                }
            };

            // player_join
        }
        _ => todo!(),
    }

    None
}
