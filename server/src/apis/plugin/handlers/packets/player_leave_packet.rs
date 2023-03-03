use std::sync::MutexGuard;

use function_name::named;
use serde::{Deserialize, Serialize};

use crate::anticheat::node_manager::NodeManager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerLeave {
    pub server_key: String,
    pub username: String,
}

#[named]
pub fn handle_player_leave(packet_data: String, mut node_manager: MutexGuard<NodeManager>) {
    let player_leave: PlayerLeave = match serde_json::from_str(&packet_data) {
        Ok(result) => result,
        Err(_) => return
    };

    match node_manager.get_node(player_leave.server_key.clone()) {
        Some(node) => {
            if node.player_exists(player_leave.username.clone()) {
                println!("Removed player: {:#?}", player_leave);
                node.remove_player(player_leave.username);
            } else {
                println!("No player found?: {:#?}", player_leave);
            }
        },
        None => {
            eprintln!("Failed to find node from server key {}", player_leave.server_key);
            eprintln!("Failed method: {}", function_name!())
        }
    }
}