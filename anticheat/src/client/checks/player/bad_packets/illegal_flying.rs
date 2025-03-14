use std::sync::Arc;
use crate::client::checks::check::{CheckInfo, CheckType, GenericCheck};
use crate::client::state::game::GameServer;
use crate::client::state::player::Player;

pub struct IllegalFlying;

impl GenericCheck for IllegalFlying {
    fn handle(player: &Player, game_server: &Arc<GameServer>) {
        if !player.allowed_flying && player.flying {
            Player::warn(player, game_server, Self::get_info());
        }
    }

    fn get_info() -> CheckInfo {
        CheckInfo {
            r#type: CheckType::Player,
            name: String::from("Illegal flying"),
            weight: 15,
            experimental: false
        }
    }
}
