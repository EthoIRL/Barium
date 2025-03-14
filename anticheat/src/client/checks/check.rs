use std::fmt::Display;
use std::sync::Arc;
use crate::client::state::game::GameServer;
use crate::client::state::player::Player;

pub trait GenericCheck {
    fn handle(player: &Player, game_server: &Arc<GameServer>);

    fn get_info() -> CheckInfo;
}

pub struct CheckInfo {
    pub r#type: CheckType,
    pub name: String,
    pub weight: i8,
    pub experimental: bool
}

#[derive(Debug)]
pub enum CheckType {
    Movement,
    Combat,
    Player,
    Other
}

impl Display for CheckType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}