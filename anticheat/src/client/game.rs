use std::collections::HashMap;
use std::sync::RwLock;
use crate::client::player::Player;
use crate::proto::generic::ServerInfo;

pub struct GameServer {
    pub info: ServerInfo,
    pub players: RwLock<HashMap<String, Player>>,
}