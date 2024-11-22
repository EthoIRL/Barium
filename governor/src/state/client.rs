use std::cmp::PartialEq;
use std::io::Write;
use std::net::TcpStream;
use uuid::Uuid;

use crate::proto::Register;
use crate::state::registration;

pub const MAXIMUM_PACKET_SIZE: u32 = 2048;

pub struct Client {
    pub stream: TcpStream,
    pub status: Status,
    pub state: Option<Register>,
    pub key: Option<Uuid>
}

#[derive(PartialEq)]
pub enum Status {
    Init,
    Registered,
    Error
}

pub fn handle_client(mut client: Client) {
    let mut data_length_buffer = [0u8; 4];

    loop {
        match &client.status {
            Status::Init => {
                match registration::handle_registration(&mut client, &mut data_length_buffer) {
                    Ok(state) => {
                        client.state = Some(state);
                        client.status = Status::Registered;
                    },
                    Err(err) => {
                        println!("[GOV] Failed to handle registration, ({:#?})", err);
                        break;
                    }
                }

                registration::handle_response(&mut client);
            },
            _ => {
                println!("{:#?}", client.state.unwrap());
                break;
            }
        }

    }
}