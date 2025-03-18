use std::sync::Arc;

use crate::client::state::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::PxPlayerCollision;

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxPlayerCollision {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let player_collision = packet.decode::<PxPlayerCollision>()?;

        if let Ok(mut player_list) = game_server.players.write() {
            let (_, player) = player_list.iter_mut().find(|(uuid, _)| uuid == &&player_collision.uuid)
                .ok_or(format!("Unknown uuid received from within sub packet, ({})", player_collision.uuid))?;

        }

        Ok(())
    }

    fn id() -> u16 {
        6
    }
}