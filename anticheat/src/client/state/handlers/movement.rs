use std::sync::Arc;
use crate::client::state::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerGround, PxPlayerMovement, PxPlayerRotation};

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerMovement {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_movement = packet.decode::<PxPlayerMovement>()?;
        println!("[{:#?} {:#?} {:#?}]", player_movement.x, player_movement.y, player_movement.z);

        if let Ok(player_list) = game_server.players.write() {
            let (_, player) = player_list.iter().find(|(uuid, _)| uuid == &&player_movement.uuid)
                .ok_or(format!("Unknown uuid received from within sub packet, ({})", player_movement.uuid))?;

            if let Ok(mut player_position) = player.position.write() {
                player_position.x = Some(player_movement.x);
                player_position.y = Some(player_movement.y);
                player_position.z = Some(player_movement.z);
            }
        }

        Ok(())
    }

    fn id() -> u16 {
        3
    }
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerRotation {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_rotation = packet.decode::<PxPlayerRotation>()?;
        println!("[{:#?} {:#?}]", player_rotation.yaw, player_rotation.pitch);

        if let Ok(player_list) = game_server.players.write() {
            let (_, player) = player_list.iter().find(|(uuid, _)| uuid == &&player_rotation.uuid)
                .ok_or(format!("Unknown uuid received from within sub packet, ({})", player_rotation.uuid))?;

            if let Ok(mut player_position) = player.position.write() {
                player_position.yaw = Some(player_rotation.yaw);
                player_position.pitch = Some(player_rotation.pitch);
            }
        }

        Ok(())
    }

    fn id() -> u16 {
        4
    }
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerGround {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_ground = packet.decode::<PxPlayerGround>()?;
        println!("Ground: [{:#?}]", player_ground.ground);

        if let Ok(player_list) = game_server.players.write() {
            let (_, player) = player_list.iter().find(|(uuid, _)| uuid == &&player_ground.uuid)
                .ok_or(format!("Unknown uuid received from within sub packet, ({})", player_ground.uuid))?;

            if let Ok(mut player_position) = player.position.write() {
                player_position.ground = Some(player_ground.ground);
            }
        }

        Ok(())
    }

    fn id() -> u16 {
        5
    }
}