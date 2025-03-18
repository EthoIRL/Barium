use std::sync::Arc;
use crate::client::checks::check::{CheckInfo, CheckType, GenericCheck};
use crate::client::state::game::GameServer;
use crate::client::state::player::Player;

pub struct ImpossibleMotion;

impl GenericCheck for ImpossibleMotion {
    fn handle(player: &Player, game_server: &Arc<GameServer>) {
        if player.locational_position.len() < 1 {
            return;
        }

        // Ignore if player is allowed to fly server side
        if player.allowed_flying {
            return;
        }

        if player.tick_data.since_join_ticks < 20 || player.tick_data.since_block_above_ticks < 10 {
            return;
        }

        match player.locational_position.get(0) {
            Some(position_0) => {
                if position_0.ground != (position_0.y % (1f64 / 64f64) == 0f64) {
                    Player::warn(player, game_server, Self::get_info());
                }
            },
            _ => return
        }
    }

    fn get_info() -> CheckInfo {
        CheckInfo {
            r#type: CheckType::Movement,
            name: String::from("Impossible Motion"),
            weight: 4,
            experimental: false
        }
    }
}
