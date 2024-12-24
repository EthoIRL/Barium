use std::io::{Error, Write};
use std::net::TcpStream;
use std::thread;

use crate::{API_VERSION, packet};
use crate::proto::anticheat::{node_registration, NodeResources};

pub fn authed_connection(governor: (&str, u16), key: &str) -> Result<TcpStream, Error> {
    let mut stream = TcpStream::connect(governor)?;

    let registration = node_registration::Register {
        node_version: API_VERSION,
        resources: Some(NodeResources {
            memory: 1,
            cpu: 1,
        }),
    };
    
    packet::send_packet(registration, 8, &mut stream);

    loop {
    }

    Ok(stream)
}