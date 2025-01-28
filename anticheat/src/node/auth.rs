use std::io::Error;
use std::net::TcpStream;
use crate::{API_VERSION, packet};
use crate::proto::anticheat::{node_registration, NodeResources};

pub fn authed_connection(stream: &mut TcpStream, key: &str) -> Result<(), Error> {
    let registration = node_registration::Register {
        node_version: API_VERSION,
        shared_key: key.to_string(),
        resources: Some(NodeResources {
            memory: 1,
            cpu: 1,
        }),
    };

    packet::send_packet(registration, 0, stream)
}