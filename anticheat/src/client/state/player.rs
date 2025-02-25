use std::sync::{Arc, RwLock};
use crate::client::state::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerJoin, PxPlayerLeave};

pub struct Player {
    pub uuid: String,
    pub name: String,
    pub position: RwLock<Position>
}

pub struct Position {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub yaw: Option<f32>,
    pub pitch: Option<f32>,
    pub ground: Option<bool>
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: None,
            y: None,
            z: None,
            yaw: None,
            pitch: None,
            ground: None
        }
    }
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerJoin {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let join_packet = packet.decode::<PxPlayerJoin>()?;

        if join_packet.uuid.is_empty() {
            return Err("Player's uuid cannot be empty... Malformed player join packet.".into());
        }

        let player = Player {
            uuid: join_packet.uuid.clone(),
            name: join_packet.name,
            position: RwLock::new(Position::default())
        };

        match game_server.players.write() {
            Ok(mut players) => {
                players.insert(join_packet.uuid.clone(), player);
                println!("[NODE] [SERVER] Player has been registered to server (Server: {}, Player: {})", game_server.key, join_packet.uuid);
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

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerLeave {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let leave_packet = packet.decode::<PxPlayerLeave>()?;

        if leave_packet.uuid.is_empty() {
            return Err("Player's uuid cannot be empty... Malformed player leave packet.".into());
        }

        match game_server.players.write() {
            Ok(mut players) => {
                players.remove(&leave_packet.uuid);
                println!("[NODE] [SERVER] Player has been removed from server (Server: {}, Player: {})", game_server.key, leave_packet.uuid);
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