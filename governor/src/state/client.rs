use std::net::TcpStream;

use crate::proto::Register;
use crate::state::registration;

pub const MAXIMUM_PACKET_SIZE: u32 = 2048;

pub struct Client {
    pub stream: TcpStream,
    pub status: Status,
    pub state: Option<Register>
}

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
            },
            _ => {
                println!("{:#?}", client.state.unwrap());
                break;
            }
        }

    }
}