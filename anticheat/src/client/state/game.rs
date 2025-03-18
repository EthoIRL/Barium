use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::client::state::player::Player;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::game::PxServerTick;
use crate::proto::generic::ServerInfo;

pub struct GameServer {
    pub relay: TcpStream,
    pub key: Uuid,
    pub info: ServerInfo,
    pub players: RwLock<HashMap<String, Player>>
}

impl GenericHandler<Arc<GameServer>, GenericPacket> for PxServerTick {
    fn handle(game_server: &mut Arc<GameServer>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut player_list) = game_server.players.write() {
            player_list.iter_mut().for_each(|(_, player)| {
            });
        }

        Ok(())
    }

    fn id() -> u16 {
        7
    }
}