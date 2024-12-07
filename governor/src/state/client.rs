use std::cmp::PartialEq;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use crate::proto::Register;
use crate::state::{packet, registration};

pub const MAXIMUM_PACKET_SIZE: u32 = 2048;

pub struct Client {
    pub stream: TcpStream,
    pub status: Status,
    pub state: Option<Register>,
    pub key: Option<Uuid>
}

#[derive(PartialEq)]
pub enum Status {
    Initialization,
    Registered,
    Crash
}

pub fn handle_client(mut client: Client) {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];
    loop {
        let packet = match packet::get_packet(&mut client.stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => data,
            Err(err) => {
                println!("[GOV] Failed to get packet, ({:#?})", err);
                return;
            }
        };

        match &client.status {
            Status::Initialization => {
                if packet.id != 0 {
                    println!("Unknown packet received during init phase");
                    return;
                }

                let authenticated = match registration::handle_registration(&mut client, packet.data) {
                    Ok(_) => {
                        true
                    },
                    Err(err) => {
                        println!("[GOV] Failed to handle registration, ({:#?})", err);
                        false
                    }
                };

                println!("Authenticated: {authenticated}");

                match registration::handle_response(&mut client, authenticated) {
                    Ok(_) => {
                        client.status = Status::Registered;
                        println!("{:#?}", client.state.unwrap());
                    }
                    Err(_) => {
                        return;
                    }
                }
            },
            _ => {
                thread::sleep(Duration::from_millis(1));
                continue;
            }
        }
    }
}