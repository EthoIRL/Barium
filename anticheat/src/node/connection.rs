use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use prost::Message;

use crate::packet;
use crate::proto::anticheat::node_registration::Response;

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

        if packet.id == 1 {
            let response = Response::decode(&*packet.data)?;

            if !response.succeeded {
                return Err(Error::new(ErrorKind::Other, "Failed to authenticate with governor server!"));
            }
        }

        if packet.id != 1 {
            println!("ID: {}", packet.id);
        }
    }

    unreachable!()
}