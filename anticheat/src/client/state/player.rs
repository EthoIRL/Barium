use std::sync::Arc;
use circular_buffer::CircularBuffer;
use crate::client::checks::check::CheckInfo;
use crate::client::state::game::GameServer;
use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerJoin, PxPlayerLeave, PxPlayerWarn};

pub struct Player {
    pub uuid: String,
    pub name: String,
    pub locational_position: CircularBuffer<20, LocationPosition>,
    pub rotational_position: CircularBuffer<20, RotationPosition>
}

pub struct LocationPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub ground: bool
}

pub struct RotationPosition {
    pub yaw: f32,
    pub pitch: f32,
}

impl Player {
    pub fn warn(&self, server: &Arc<GameServer>, check_info: CheckInfo) {
        packet::send_proxied_packet(PxPlayerWarn {
            uuid: self.uuid.clone(),
            check_type: check_info.r#type.to_string(),
            check_name: check_info.name,
            check_weight: check_info.weight as i32,
            check_experimental: check_info.experimental
        }, 0, &mut server.relay.try_clone().unwrap()).unwrap();
    }

    pub fn lag_back(&self, location_position: LocationPosition, rotation_position: RotationPosition) {
        todo!()
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
            locational_position: CircularBuffer::new(),
            rotational_position: CircularBuffer::new()
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