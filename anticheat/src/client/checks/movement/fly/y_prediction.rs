use std::sync::Arc;
use crate::client::checks::check::{CheckInfo, CheckType, GenericCheck};
use crate::client::state::game::GameServer;
use crate::client::state::player::Player;

const MOVEMENT_MULTIPLIER: f64 = 0.9800000190734863;
const ENTITY_GRAVITY: f64 = 0.08;
pub struct YPrediction;

impl GenericCheck for YPrediction {
    fn handle(player: &Player, game_server: &Arc<GameServer>) {
        // Only start check after 4 positions have accumulated
        if player.locational_position.len() < 4 {
            return;
        }

        // Ignore if player is allowed to fly server side
        if player.allowed_flying {
            return;
        }

        // Get the last 4 player positions
        match (
            player.locational_position.get(0),
            player.locational_position.get(1),
            player.locational_position.get(2),
            player.locational_position.get(3)
        ) {
            (Some(position_0), Some(position_1), Some(position_2), Some(position_3)) => {
                let delta_y_1 = position_0.y - position_1.y;
                let delta_y_2 = position_2.y - position_3.y;

                // Must be off ground
                if position_0.ground || position_1.ground ||
                    position_2.ground || position_3.ground {
                    return;
                }

                let delta_difference = delta_y_1 - ((delta_y_2 - ENTITY_GRAVITY) * MOVEMENT_MULTIPLIER);

                // TODO: Block above head must be checked
                if delta_difference.abs() >= 0.1 || delta_difference == 0.0784000015258789f64 {
                    Player::warn(player, game_server, Self::get_info());
                }
            },
            _ => return
        }
    }

    fn get_info() -> CheckInfo {
        CheckInfo {
            r#type: CheckType::Movement,
            name: String::from("Y Prediction"),
            weight: 5,
            experimental: true
        }
    }
}
