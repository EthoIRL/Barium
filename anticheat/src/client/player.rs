use std::sync::Arc;
use crate::client::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::PxPlayerJoin;

pub struct Player {
    uuid: String,
    name: String
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for Player {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_packet = packet.decode::<PxPlayerJoin>()?;

        if player_packet.uuid.is_empty() {
            return Err("Player's uuid cannot be empty... Malformed player join packet.".into());
        }

        let player = Player {
            uuid: player_packet.uuid.clone(),
            name: player_packet.name
        };

        match game_server.players.write() {
            Ok(mut players) => {
                players.insert(player_packet.uuid, player);

                // println!("[NODE] [SERVER] New player has been registered to server ({}, {})")
            },
            Err(err) => {
                return Err(err.to_string().into());
            }
        }

        Ok(())
    }

    fn id() -> u16 {
        0
    }
}