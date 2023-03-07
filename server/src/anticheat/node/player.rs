use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodePlayer {
    pub username: String,
    pub uuid: Option<String>,
    pub version: Option<String>,
    pub player_data: PlayerData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerData {
    pub x: f64,
    pub y: f64, 
    pub z: f64,
    pub ground: bool
}

impl PlayerData {
    pub fn default() -> Self {
        PlayerData {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            ground: true
        }
    }
}