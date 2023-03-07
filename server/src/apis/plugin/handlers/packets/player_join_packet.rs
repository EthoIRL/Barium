use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

use function_name::named;
use crate::anticheat::node_manager::NodeManager;
use crate::anticheat::node::player::{NodePlayer, PlayerData};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerJoin {
    pub server_key: String,
    pub username: String,
    pub uuid: Option<String>,
    pub version: Option<String>,
}

#[named]
pub fn handle_player_join(packet_data: String, mut node_manager: MutexGuard<NodeManager>) {
    let player_join: PlayerJoin = match serde_json::from_str(&packet_data) {
        Ok(result) => result,
        Err(_) => return
    };

    match node_manager.get_node(player_join.server_key.clone()) {
        Some(node) => {
            let node_player = NodePlayer {
                username: player_join.username,
                uuid: player_join.uuid,
                version: player_join.version,
                player_data: PlayerData::default(),
            };

            println!("PLAYER JOINED: {:#?}", node_player);

            node.add_player(node_player);
        }
        None => {
            eprintln!("Failed to find node from server key {}", player_join.server_key);
            eprintln!("Failed method: {}", function_name!())
        }
    };
}