use crate::{API_VERSION, NODE_KEY, packet};
use crate::error::RegistrationError;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::anticheat::node_registration;
use crate::proto::generic::DisconnectReason;
use crate::server::node;
use crate::server::node::Node;

pub struct NodeRegistar;

impl GenericHandler<Node, GenericPacket> for NodeRegistar {
    fn handle(node: &mut Node, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let registration_packet = packet.decode::<node_registration::Register>()?;

        if registration_packet.node_version != API_VERSION {
            node::disconnect_node(node, DisconnectReason::Unknown);
            return Err(RegistrationError::MismatchVersion {
                supplied_version: registration_packet.node_version
            }.into());
        }

        if registration_packet.shared_key != NODE_KEY {
            node::disconnect_node(node, DisconnectReason::Unknown);
            return Err(RegistrationError::BadResponse.into());
        }

        node.resources = registration_packet.resources;

        let response = node_registration::Response {
            succeeded: true,
        };

        if let Err(err) = packet::send_packet(response, 1, &mut node.stream) {
            node::disconnect_node(node, DisconnectReason::Unknown);
            return Err(err.into());
        };

        Ok(())
    }
}