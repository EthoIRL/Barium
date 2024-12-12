use std::{thread, u16};
use std::io::Error;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use prost::Message;
use uuid::Uuid;
use crate::proto::generic::DisconnectReason;
use crate::proto::server::DisconnectServer;
use crate::proto::server::server_registration::Register;
use crate::state::{packet, registration};

use crate::state::registration::RegistrationError;

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

pub fn start_plugin_server(address: (&str, u16)) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>, Error> {
    let listener = TcpListener::bind(address).unwrap();

    let thread_pool: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));
    let pool = thread_pool.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(tcp_stream) = stream {
                let peer_address = match tcp_stream.peer_addr() {
                    Ok(addr) => addr.ip(),
                    Err(err) => {
                        eprintln!("Failed to get peer address: ({err})");
                        continue;
                    }
                };

                println!("[GOV] Incoming connection from ({})", peer_address.to_string());

                let client = Client {
                    stream: tcp_stream,
                    status: Status::Initialization,
                    state: None,
                    key: None,
                    ip_addr: peer_address
                };

                if let Ok(mut pool) = pool.lock() {
                    pool.push(thread::spawn(|| handle_client(client)));
                }
            }
        }
    }); 

    Ok(thread_pool)
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
            disconnect_client(&mut client, DisconnectReason::Unknown);
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
                    disconnect_client(&mut client, DisconnectReason::Unknown);
                    eprintln!("Unknown packet received during init phase");
                    return;
                }

                if let Err(err) = registration::handle_registration(&mut client, packet.data) {
                    println!("Failed to authenticate client, (Reason: {:#?})", err);

                    disconnect_client(&mut client, match err {
                        RegistrationError::MismatchVersion => DisconnectReason::MismatchVersion,
                        _ => DisconnectReason::Unknown
                    });

                    return;
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
    let disconnect = match DisconnectServer::decode(&*packet_data) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error decoding disconnect");
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

fn disconnect_client(client: &mut Client, reason: DisconnectReason) {
    let disconnect_packet = DisconnectServer {
        uuid_key: match client.key {
            Some(key) => Some(key.to_string()),
            None => None
        },
        reason: i32::from(reason)
    };

    
    let _ = packet::send_packet(disconnect_packet, 2, &mut client.stream);
}