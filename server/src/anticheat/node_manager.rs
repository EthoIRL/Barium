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

    pub fn remove_node(&mut self, key: String) {
        let index = match self.find_index(key) {
            Some(id) => id,
            None => return
        };

        self.nodes.remove(index);
    }

    pub fn get_node(&mut self, key: String) -> Option<&mut Node> {
        let index = match self.find_index(key) {
            Some(id) => id,
            None => return None
        };

        match self.nodes.get_mut(index) {
            Some(node) => Some(node),
            None => None,
        }
    }

    fn find_index(&self, key: String) -> Option<usize> {
        match self.nodes.iter().position(|x| x.key == key) {
            Some(id) => Some(id),
            None => {
                eprintln!("Failed to find node by key ({})", key);
                return None
            }
        }
    }
}
