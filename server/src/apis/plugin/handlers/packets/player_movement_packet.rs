use std::sync::MutexGuard;

use function_name::named;
use serde::{Deserialize, Serialize};

use crate::anticheat::node_manager::NodeManager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerMovement {
    pub server_key: String,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub ground: bool
}

#[named]
pub fn handle_player_movement(packet_data: String, mut node_manager: MutexGuard<NodeManager>) {
    let player_movement: PlayerMovement = match serde_json::from_str(&packet_data) {
        Ok(result) => result,
        Err(_) => return
    };

    match node_manager.get_node(player_movement.server_key.clone()) {
        Some(node) => {
            if node.player_exists(player_movement.username.clone()) {
                let player = match node.get_player(player_movement.username.clone()) {
                    Some(player) => player,
                    None => return
                };

                player.player_data.x = player_movement.x;
                player.player_data.y = player_movement.y;
                player.player_data.z = player_movement.z;
                player.player_data.ground = player_movement.ground;

                println!("Player: {}, X: {}, Y: {}, Z: {}, Ground: {}", player_movement.username, player_movement.x, player_movement.y, player_movement.z, player_movement.ground)
            } else {
                println!("No player found?: {:#?}", player_movement);
            }
        },
        None => {
            eprintln!("Failed to find node from server key {}", player_movement.server_key);
            eprintln!("Failed method: {}", function_name!())
        }
    }
}