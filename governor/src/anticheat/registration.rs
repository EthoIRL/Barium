use std::io::Error;
use crate::packet::{GenericHandler, GenericPacket};
use crate::server::node::Node;

pub struct NodeRegistar;

impl GenericHandler<Node> for NodeRegistar {
    fn handle(node: &mut Node, packet: GenericPacket) -> Result<(), Error> {
        todo!()
    }
}