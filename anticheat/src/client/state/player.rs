use std::sync::Arc;
use crate::client::state::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerJoin, PxPlayerLeave};

pub struct Player {
    uuid: String,
    name: String
}

pub struct PlayerJoin {}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PlayerJoin {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let join_packet = packet.decode::<PxPlayerJoin>()?;

        if join_packet.uuid.is_empty() {
            return Err("Player's uuid cannot be empty... Malformed player join packet.".into());
        }

        let player = Player {
            uuid: join_packet.uuid.clone(),
            name: join_packet.name
        };

        match game_server.players.write() {
            Ok(mut players) => {
                players.insert(join_packet.uuid.clone(), player);
                println!("[NODE] [SERVER] New player has been registered to server (Server: {}, Player: {})", game_server.key, join_packet.uuid);
            },
            Err(err) => {
                return Err(err.to_string().into());
            }
        }

        Ok(())
    }

    fn id() -> u16 {
        1
    }
}

pub struct PlayerLeave {}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PlayerLeave {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let leave_packet = packet.decode::<PxPlayerLeave>()?;

        if leave_packet.uuid.is_empty() {
            return Err("Player's uuid cannot be empty... Malformed player leave packet.".into());
        }

        match game_server.players.write() {
            Ok(mut players) => {
                players.remove(&leave_packet.uuid);
                println!("[NODE] [SERVER] New player has been removed from the server (Server: {}, Player: {})", game_server.key, leave_packet.uuid);
            },
            Err(err) => {
                return Err(err.to_string().into());
            }
        }

        Ok(())
    }

    fn id() -> u16 {
        2
    }
}