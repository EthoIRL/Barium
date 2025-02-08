use std::collections::HashMap;
use crate::client::player::Player;
use crate::proto::generic::ServerInfo;

pub struct GameServer {
    pub info: ServerInfo,
    pub players: HashMap<String, Player>,
}