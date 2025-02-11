use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use crate::client::player::Player;
use crate::proto::generic::ServerInfo;

pub struct GameServer {
    pub key: Uuid,
    pub info: ServerInfo,
    pub players: RwLock<HashMap<String, Player>>,
}