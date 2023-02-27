pub mod node_manager {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Node {
        // pub barium_key: String,
        // pub server_hardware_id: String,
        // pub server_ip: String,

        pub server_version: String,
        // pub server_region: Region,
        pub via_version: bool,
        pub bungee_cord: bool,
        pub cracked: bool,
        pub key: String
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum Region {
        NA, // North America
        EU, // Europe
        AS, // Asia
        AF, // Africa
        OC, // Oceania
        SA, // South America
        AN // Antarctica WTF?
    }
    
    #[derive(Clone)]
    pub struct NodeManager {
        pub nodes: Vec<Node>
    }

    impl NodeManager {
        pub fn default() -> Self {
            // let nodes: Vec<Node> = Vec::new();
            NodeManager { 
                nodes: Vec::new()
            }
        }

        pub fn add_node(&mut self, node: Node) {
            self.nodes.push(node);
        }

        pub fn get_node(&self, key: String) -> Option<&Node> {
            self.nodes.iter().find(|x| x.key == key)
        }
    }
}