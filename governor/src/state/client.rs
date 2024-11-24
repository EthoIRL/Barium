use std::cmp::PartialEq;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
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
                let authenticated = match registration::handle_registration(&mut client, &mut data_length_buffer) {
                    Ok(state) => {
                        client.state = Some(state);
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