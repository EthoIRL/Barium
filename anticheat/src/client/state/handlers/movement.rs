use std::sync::Arc;
use crate::client::state::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerMovement, PxPlayerRotation};

pub struct PlayerMovement {}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PlayerMovement {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_movement = packet.decode::<PxPlayerMovement>()?;
        println!("[{:#?} {:#?} {:#?}] [{:#?}]", player_movement.x, player_movement.y, player_movement.z, player_movement.ground);

        Ok(())
    }

    fn id() -> u16 {
        3
    }
}

