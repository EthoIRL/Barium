use std::sync::Arc;
use crate::client::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};

pub struct Player {
    uuid: String,
    name: String
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for Player {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn id() -> u16 {
        0
    }
}