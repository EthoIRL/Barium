use std::{thread, u16};
use std::collections::HashMap;
use std::io::Error;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;
use uuid::Uuid;
use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::plugin::disconnect::ClientDisconnect;
use crate::plugin::proxy::ClientProxy;
use crate::plugin::registration::ClientRegistration;
use crate::proto::generic::DisconnectReason;
use crate::proto::server;
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
    pub node_stream: Option<TcpStream>,
    pub node_id: Option<Uuid>
}

#[derive(PartialEq)]
pub enum Status {
    Initialization,
    Registered,
    Ready,
    Crash,
}

pub fn start_plugin_server(address: (&str, u16), node_list: Arc<RwLock<HashMap<Uuid, Arc<Node>>>>) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>, Error> {
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
                    connected: true,
                    node_stream: None,
                    node_id: None
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

pub fn handle_client(mut client: Client, node_list: Arc<RwLock<HashMap<Uuid, Arc<Node>>>>) {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    let mut known_packets: HashMap<u16, fn(&mut Client, GenericPacket) -> Result<(), Box<dyn std::error::Error>>> = HashMap::new();
    known_packets.insert(0, ClientRegistration::handle);
    known_packets.insert(2, ClientDisconnect::handle);
    known_packets.insert(4, ClientProxy::handle);

    loop {
        if !client.connected {
            return;
        }

        if client.status == Status::Registered {
            if let Ok(node_list) = node_list.read() {
                if node_list.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }

                // TODO: Pick node based on resources & current clients connected

                let node = match node_list.iter().next() {
                    Some(node) => node.1,
                    None => continue
                };

                let stream = match node.stream.try_clone() {
                    Ok(stream) => stream,
                    Err(err) => {
                        println!("[GOV] [CLIENT] Failed to access node stream, ({:#?})", err);
                        disconnect_client(&mut client, DisconnectReason::Crash);
                        return;
                    }
                };

                client.node_stream = Some(stream);
                client.node_id = Some(node.id);

                if let Err(err) = packet::send_packet(server::Ready::default(), 3, &mut client.stream) {
                    println!("[GOV] [CLIENT] Failed to send ready packet, ({:#?})", err);
                    disconnect_client(&mut client, DisconnectReason::Unknown);
                    return;
                };

                println!("[GOV] [CLIENT] Client registered to anticheat server!");

                client.status = Status::Ready;
            }
        }

        let packet = match packet::get_packet(&mut client.stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => {
                if client.status == Status::Ready {
                    assert!(client.node_id.is_some(), "[GOV] [CLIENT] Client does not have node id tied to client after Ready status!");

                    if let Ok(node_list) = node_list.read() {
                        if !node_list.contains_key(&client.node_id.unwrap()) {
                            println!("[GOV] [CLIENT] Node no longer exists; disconnecting client");
                            return;
                        }
                    }
                }

                data
            },
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