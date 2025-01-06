use std::io::Error;
use crate::packet::{GenericHandler, GenericPacket};
use crate::server::node::Node;

pub struct NodeRegistar;

impl GenericHandler<Node, GenericPacket> for NodeRegistar {
    fn handle(node: &mut Node, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}