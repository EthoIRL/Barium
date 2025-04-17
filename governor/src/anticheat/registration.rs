use std::sync::Arc;
use rsa::Pkcs1v15Encrypt;
use crate::{API_VERSION, packet};
use crate::error::RegistrationError;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::anticheat::node_registration;
use crate::proto::generic::DisconnectReason;
use crate::server::node;
use crate::server::node::{Node, NodeStatus};

pub struct NodeRegistar;

impl GenericHandler<Arc<Node>, GenericPacket> for NodeRegistar {
    fn handle(node: &mut Arc<Node>, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let registration_packet = packet.decode::<node_registration::Register>()?;

        if registration_packet.node_version != API_VERSION {
            node::disconnect_node(node, DisconnectReason::Unknown);
            return Err(RegistrationError::MismatchVersion {
                supplied_version: registration_packet.node_version
            }.into());
        }

        let decoded_key = encoding_rs::mem::encode_latin1_lossy(&registration_packet.shared_key);

        let shared_key_vec = match node.private_key.decrypt(Pkcs1v15Encrypt, &*decoded_key) {
            Ok(key) => key,
            Err(err) => {
                println!("[GOV] [NODE] Failed to decrypt shared key ({})", err);
                return Err(RegistrationError::BadResponse.into());
            }
        };

        let remote_shared_key = match String::from_utf8(shared_key_vec) {
            Ok(key) => key,
            Err(err) => {
                println!("[GOV] [NODE] Failed to transform bytes to string during key decryption phase. ({})", err);
                return Err(RegistrationError::BadResponse.into());
            }
        };

        let shared_key_status = remote_shared_key == node.shared_key;

        let response = node_registration::Response {
            succeeded: shared_key_status,
        };

        if let Err(err) = packet::send_packet(response, 1, &mut node.origin_stream.try_clone().unwrap()) {
            node::disconnect_node(node, DisconnectReason::Unknown);
            return Err(err.into());
        };

        match shared_key_status {
            true => {
                println!("[GOV] [NODE] Authenticated node ({}, {})", node.ip_addr.to_string(), node.id.to_string());

                if let Ok(mut resources) = node.resources.lock() {
                    *resources = registration_packet.resources;
                }

                if let Ok(mut status) = node.status.write() {
                    *status = NodeStatus::Ready;
                }
            },
            false => {
                node::disconnect_node(node, DisconnectReason::Unknown);
            }
        }

        Ok(())
    }
}