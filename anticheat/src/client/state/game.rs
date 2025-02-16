use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use crate::client::state::player::Player;
use crate::proto::generic::ServerInfo;

pub struct GameServer {
    pub key: Uuid,
    pub info: ServerInfo,
    pub players: RwLock<HashMap<String, Player>>
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {
            key: Uuid::default(),
            info: ServerInfo::default(),
            players: RwLock::new(HashMap::new())
        }
    }
}