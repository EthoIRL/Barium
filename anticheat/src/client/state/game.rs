use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::RwLock;
use uuid::Uuid;
use crate::client::state::player::Player;
use crate::proto::generic::ServerInfo;

pub struct GameServer {
    pub relay: TcpStream,
    pub key: Uuid,
    pub info: ServerInfo,
    pub players: RwLock<HashMap<String, Player>>
}