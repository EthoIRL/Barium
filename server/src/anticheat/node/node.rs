use serde::{Deserialize, Serialize};

use crate::anticheat::node::player::NodePlayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    // pub barium_key: String,
    pub server_ip: String,
    pub server_version: String,
    pub server_region: Region,
    pub server_os: String,
    pub via_version: bool,
    pub key: String,
    pub players: Vec<NodePlayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Region {
    NA, // North America
    EU, // Europe
    AS, // Asia
    AF, // Africa
    OC, // Oceania
    SA, // South America
    AN, // Antarctica WTF?
}

impl Node {
    pub fn new(
        server_ip: String,
        server_version: String,
        server_region: Region,
        server_os: String,
        via_version: bool,
        key: String,
    ) -> Self {
        Node {
            server_ip: server_ip,
            server_version: server_version,
            server_region: server_region,
            server_os: server_os,
            via_version: via_version,
            key: key,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, node_player: NodePlayer) {
        self.players.push(node_player);
    }

    pub fn get_player(&mut self, username: String) -> Option<&mut NodePlayer> {
        let index = match self.find_index(username) {
            Some(id) => id,
            None => return None
        };

        match self.players.get_mut(index) {
            Some(player) => Some(player),
            None => None,
        }
    }

    pub fn player_exists(&mut self, username: String) -> bool {
        self.players.iter().any(|x| x.username == username)
    }

    pub fn remove_player(&mut self, username: String) {
        let index = match self.find_index(username) {
            Some(id) => id,
            None => return
        };

        self.players.remove(index);
    }

    fn find_index(&self, username: String) -> Option<usize> {
        match self.players.iter().position(|x| x.username == username) {
            Some(id) => Some(id),
            None => None
        }
    }
}
