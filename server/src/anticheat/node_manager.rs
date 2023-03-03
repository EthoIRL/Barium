use crate::anticheat::node::node::Node;

#[derive(Clone)]
pub struct NodeManager {
    pub nodes: Vec<Node>,
}

impl NodeManager {
    pub fn default() -> Self {
        NodeManager { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn get_node(&mut self, key: String) -> Option<&mut Node> {
        let index = match self.nodes.iter().position(|x| x.key == key) {
            Some(id) => id,
            None => {
                eprintln!("Failed to get node key dex ({})", key);
                return None;
            }
        };

        match self.nodes.get_mut(index) {
            Some(node) => Some(node),
            None => None,
        }
    }
}
