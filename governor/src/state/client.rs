use std::cmp::PartialEq;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use prost::Message;
use uuid::Uuid;

use crate::proto::{Disconnect, DisconnectReason, Register};
use crate::state::{packet, registration};

pub const MAXIMUM_PACKET_SIZE: usize = 2048;

pub struct Client {
    pub stream: TcpStream,
    pub status: Status,
    pub state: Option<Register>,
    pub key: Option<Uuid>,
    pub ip_addr: IpAddr
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

        if packet.data.len() > MAXIMUM_PACKET_SIZE {
            
            return;
        }

        if packet.id == 2 {
            match handle_disconnect(&mut client, packet.data) {
                (disconnect, reason) => {
                    if disconnect {
                        println!("[GOV] Client disconnected, Reason: ({:#?})", reason);
                        return;
                    }

                    eprintln!("Disconnect packet id received; but failed to disconnect!");
                    return;
                }
            }
        }

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

pub fn handle_disconnect(client: &mut Client, packet_data: Vec<u8>) -> (bool, DisconnectReason) {
    let disconnect = match Disconnect::decode(&*packet_data) {
        Ok(data) => data,
        Err(_) => {
            println!("Error decoding disconnect");
            return (false, DisconnectReason::Unknown);
        }
    };

    if client.status != Status::Initialization {
        let packet_uuid = match &disconnect.uuid_key {
            Some(uuid) => {
                match Uuid::from_str(&*uuid) {
                    Ok(uuid) => uuid,
                    Err(_) => {
                        return (false, DisconnectReason::Unknown);
                    }
                }
            },
            None => {
                return (false, DisconnectReason::Unknown);
            }
        };

        let server_uuid = match client.key {
            Some(uuid) => uuid,
            None => {
                eprintln!("[GOV] Server key invalid past Initialization phase.");
                return (false, DisconnectReason::Unknown);
            }
        };

        if server_uuid != packet_uuid {
            return (false, DisconnectReason::Unknown);
        }
    }

    return (true, disconnect.reason());
}