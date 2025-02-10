use crate::client::game::GameServer;
use crate::packet::{GenericHandler, GenericPacket};

pub struct Player {
    uuid: String,
    name: String
}

impl GenericHandler<GameServer, GenericPacket> for Player {
    fn handle(game_server: &mut GameServer, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn id() -> u16 {
        0
    }
}