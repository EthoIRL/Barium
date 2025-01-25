use std::{thread, u16};
use std::collections::HashMap;
use std::io::Error;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use uuid::Uuid;
use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::plugin::disconnect::ClientDisconnect;
use crate::plugin::registration::ClientRegistration;
use crate::proto::generic::DisconnectReason;
use crate::proto::server::DisconnectServer;
use crate::proto::server::server_registration::Register;
use crate::server::node::Node;

pub const MAXIMUM_PACKET_SIZE: usize = 2048;

pub struct Client {
    pub stream: TcpStream,
    pub status: Status,
    pub state: Option<Register>,
    pub key: Option<Uuid>,
    pub ip_addr: IpAddr,
    pub connected: bool,
}

#[derive(PartialEq)]
pub enum Status {
    Initialization,
    Registered,
    Ready,
    Crash,
}

pub fn start_plugin_server(address: (&str, u16), node_list: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>, Error> {
    let listener = TcpListener::bind(address)?;

    let thread_pool: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));
    let pool = thread_pool.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(tcp_stream) = stream {
                let peer_address = match tcp_stream.peer_addr() {
                    Ok(addr) => addr.ip(),
                    Err(err) => {
                        eprintln!("[GOV] [CLIENT] Failed to get peer address: ({err})");
                        continue;
                    }
                };

                println!("[GOV] [CLIENT] Incoming connection from ({})", peer_address.to_string());

                let client = Client {
                    stream: tcp_stream,
                    status: Status::Initialization,
                    state: None,
                    key: None,
                    ip_addr: peer_address,
                    connected: true
                };

                let list = node_list.clone();

                if let Ok(mut pool) = pool.lock() {
                    pool.push(thread::spawn(move || handle_client(client, list)));
                }
            }
        }
    });

    Ok(thread_pool)
}

pub fn handle_client(mut client: Client, node_list: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>) {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    let mut known_packets: HashMap<u16, fn(&mut Client, GenericPacket) -> Result<(), Box<dyn std::error::Error>>> = HashMap::new();
    known_packets.insert(0, ClientRegistration::handle);
    known_packets.insert(2, ClientDisconnect::handle);

    loop {
        if !client.connected {
            return;
        }

        let packet = match packet::get_packet(&mut client.stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => data,
            Err(err) => {
                println!("[GOV] [CLIENT] Failed to get packet, ({:#?})", err);
                disconnect_client(&mut client, DisconnectReason::Crash);
                return;
            }
        };

        if packet.data.len() > MAXIMUM_PACKET_SIZE {
            disconnect_client(&mut client, DisconnectReason::Unknown);
            return;
        }

        if client.status == Status::Initialization {
            if packet.id != 0 {
                disconnect_client(&mut client, DisconnectReason::Unknown);
                return;
            }
        }

        let packet_handle = match known_packets.get(&packet.id) {
            Some(handler) => handler,
            None => {
                println!("[GOV] [CLIENT] PacketID not found, ({})", packet.id);
                continue;
            }
        };

        packet_handle(&mut client, packet).unwrap();
    }

    unreachable!()
}

pub fn disconnect_client(client: &mut Client, reason: DisconnectReason) {
    println!("[GOV] [CLIENT] Client disconnected, Reason: ({:#?})", reason);

    client.connected = false;

    let disconnect_packet = DisconnectServer {
        uuid_key: match client.key {
            Some(key) => Some(key.to_string()),
            None => None
        },
        reason: i32::from(reason),
    };

    let _ = packet::send_packet(disconnect_packet, 2, &mut client.stream);
}