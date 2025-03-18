use std::sync::Arc;
use crate::client::checks::check::GenericCheck;
use crate::client::checks::movement::fly::y_prediction::YPrediction;
use crate::client::state::game::GameServer;
use crate::client::state::player::{LocationPosition, RotationPosition};
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::{PxPlayerMovement, PxPlayerRotation};

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerMovement {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_movement = packet.decode::<PxPlayerMovement>()?;
        if let Ok(mut player_list) = game_server.players.write() {
            let (_, player) = player_list.iter_mut().find(|(uuid, _)| uuid == &&player_movement.uuid)
                .ok_or(format!("Unknown uuid received from within sub packet, ({})", player_movement.uuid))?;

            player.locational_position.push_front(LocationPosition {
                x: player_movement.x,
                y: player_movement.y,
                z: player_movement.z,
                ground: player_movement.ground
            });

            for (_, player) in player_list.iter() {
                YPrediction::handle(player, &game_server);
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
        if let Ok(mut player_list) = game_server.players.write() {
            let (_, player) = player_list.iter_mut().find(|(uuid, _)| uuid == &&player_rotation.uuid)
                .ok_or(format!("Unknown uuid received from within sub packet, ({})", player_rotation.uuid))?;

            player.rotational_position.push_front(RotationPosition {
                yaw: player_rotation.yaw,
                pitch: player_rotation.pitch
            })
        }

        Ok(())
    }

    fn id() -> u16 {
        4
    }
}