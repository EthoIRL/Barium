pub mod player {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NodePlayer {
        pub username: String,
        pub uuid: Option<String>,
        pub version: Option<String>,
        pub player_data: Option<PlayerData>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PlayerData {

    }
}