use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use prost::Message;

use crate::{API_VERSION, packet};
use crate::proto::anticheat::{node_registration, NodeResources};
use crate::proto::anticheat::node_registration::Response;

pub fn authed_connection(stream: &mut TcpStream, key: &str) -> Result<(), Error> {
    let registration = node_registration::Register {
        node_version: API_VERSION,
        shared_key: key.to_string(),
        resources: Some(NodeResources {
            memory: 1,
            cpu: 1,
        }),
    };

    packet::send_packet(registration, 0, stream)?;

    handle_response(stream)
}

pub fn handle_response(stream: &mut TcpStream) -> Result<(), Error> {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    loop {
        let packet = match packet::get_packet(stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => {
                data
            }
            Err(err) => {
                return Err(err);
            }
        };

        assert_eq!(packet.id, 1, "Unknown packet received during registration response phase");

        if packet.id != 1 {
            return Err(Error::new(ErrorKind::Other, "Unknown packet received during registration response phase"));
        }

        let response = Response::decode(&*packet.data)?;

        if response.succeeded {
            return Ok(());
        }

        return Err(Error::new(ErrorKind::Other, "Failed to authenticate with governor server!"));
    }
}