use std::sync::Arc;
use crate::client::state::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerGround, PxPlayerMovement, PxPlayerRotation};

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerMovement {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_movement = packet.decode::<PxPlayerMovement>()?;
        println!("[{:#?} {:#?} {:#?}]", player_movement.x, player_movement.y, player_movement.z);

        Ok(())
    }

    fn id() -> u16 {
        3
    }
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerRotation {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_movement = packet.decode::<PxPlayerRotation>()?;
        println!("[{:#?} {:#?}]", player_movement.yaw, player_movement.pitch);

        Ok(())
    }

    fn id() -> u16 {
        4
    }
}