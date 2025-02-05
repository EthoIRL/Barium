use std::collections::HashMap;
use crate::client::player::Player;

pub struct GameServer {
    pub players: HashMap<String, Player>,
}