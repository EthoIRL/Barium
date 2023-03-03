#[allow(non_snake_case)]
pub mod Play {
    use std::{
        sync::{Arc, Mutex}, f32::consts::E,
    };

    use serde::{Serialize, Deserialize};

    use crate::{apis::plugin::packets::packet::packet::Packet, anticheat::node::player::player::NodePlayer};
    use crate::NodeManager;

    use crate::apis::plugin::packets::packet::packet::{to_string, to_buffer};
    use crate::anticheat::node::node::node::{Node, Region};

    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PlayerJoin {
        pub server_key: String,
        pub username: String,
        pub uuid: Option<String>,
        pub version: Option<String>
    }

    pub fn handlePlayPacket(packet: Packet, node_manager_arc: &Arc<Mutex<NodeManager>>) -> Option<Vec<u8>> {
        let id = packet.id;
        let packet_data: String = match to_string(packet) {
            Some(data) => data,
            None => {
                panic!("No string data found in packet!")
            }
        };

        let mut node_manager = match node_manager_arc.lock() {
            Ok(result) => result,
            Err(_) => {
                return None
            }
        };
        
        match id as i8 {
            10 => {
                let player_join: PlayerJoin = match serde_json::from_str(&packet_data) {
                    Ok(result) => result,
                    Err(_) => return None
                };

                match node_manager.get_node(player_join.server_key.clone()) {
                    Some(node) => {
                        let node_player = NodePlayer {
                            username: player_join.username,
                            uuid: player_join.uuid,
                            version: player_join.version,
                            player_data: None
                        };

                        println!("PLAYER JOINED: {:#?}", node_player);

                        node.add_player(node_player);
                    },
                    None => {
                        eprintln!("Failed to find node from server key {}" , player_join.server_key)
                    }
                };
                
                // player_join
            }
            _ => todo!()
        }
        
        None
    }
}