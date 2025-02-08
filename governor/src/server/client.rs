use std::{thread, u16};
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind::ConnectionReset;
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
use crate::proto::anticheat::NodeProxyNegotiation;
use crate::proto::generic::DisconnectReason;
use crate::proto::server;
use crate::proto::server::DisconnectServer;
use crate::proto::server::server_registration::Register;
use crate::server::node::{Node, NodeStatus};

pub const MAXIMUM_PACKET_SIZE: usize = 2048;

pub struct Client {
    pub stream: TcpStream,
    pub status: ClientStatus,
    pub state: Option<Register>,
    pub key: Option<Uuid>,
    pub ip_addr: IpAddr,
    pub connected: Arc<RwLock<bool>>,
    pub node_stream: Option<TcpStream>,
    pub node_id: Option<Uuid>,
}

#[derive(PartialEq)]
pub enum ClientStatus {
    Initialization,
    Registered,
    Ready
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
                    status: ClientStatus::Initialization,
                    state: None,
                    key: None,
                    ip_addr: peer_address,
                    connected: Arc::new(RwLock::new(true)),
                    node_stream: None,
                    node_id: None,
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
    known_packets.insert(10, ClientProxy::handle);

    loop {
        if let Ok(connection) = client.connected.read() {
            if !*connection {
                return;
            }
        }

        if client.status == ClientStatus::Registered {
            if let Ok(node_list) = node_list.read() {
                if node_list.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }

                // TODO: Pick node based on resources & current clients connected
                let node = match node_list.iter().find(|(_, node)| {
                    if let Ok(status) = node.status.read() {
                        if *status == NodeStatus::Ready {
                            return true;
                        }
                    }
                    false
                }) {
                    Some(node) => node.1,
                    None => {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                };

                if let Err(err) = negotiate_node_registration(node, &mut client) {
                    println!("[GOV] [CLIENT] {err}");
                    disconnect_client(&mut client, DisconnectReason::Crash);
                }
            }
        }

        let packet = match packet::get_packet(&mut client.stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => {
                if client.status == ClientStatus::Ready {
                    assert!(client.node_id.is_some(), "[GOV] [CLIENT] Client does not have node id tied to client after Ready status!");

                    if let Ok(node_list) = node_list.read() {
                        if !node_list.contains_key(&client.node_id.unwrap()) {
                            println!("[GOV] [CLIENT] Node no longer exists; disconnecting client");
                            return;
                        }
                    }
                }

                data
            }
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

        if client.status == ClientStatus::Initialization {
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

pub fn negotiate_node_registration(node: &Arc<Node>, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut origin_stream = match node.origin_stream.try_clone() {
        Ok(stream) => stream,
        Err(err) => {
            return Err(format!("Failed to access node stream, ({:#?})", err).into());
        }
    };

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(err) => {
            return Err(format!("Failed to create proxy listener, ({:#?})", err).into());
        }
    };

    let stream_port = match listener.local_addr() {
        Ok(local_addr) => local_addr.port() as u32,
        Err(err) => {
            return Err(format!("Failed to retrieve port from proxy listener, ({:#?})", err).into());
        }
    };

    assert!(client.key.is_some(), "Client key is empty past registration phase.");
    assert!(client.state.is_some(), "Client state is empty past registration phase.");

    let proxy_negotiation = NodeProxyNegotiation {
        port: stream_port,
        client_key: client.key.unwrap().to_string(),
        server_info: client.state.unwrap().server_info,
    };

    if let Err(err) = packet::send_packet(proxy_negotiation, 3, &mut origin_stream) {
        return Err(format!("Failed to send packet to origin node stream, ({:#?})", err).into());
    };

    let proxy_connection = match listener.accept() {
        Ok(listener) => listener.0,
        Err(err) => {
            return Err(format!("Node failed to connect to client proxy stream, ({:#?})", err).into());
        }
    };
    
    node_client_relay(client.connected.clone(), proxy_connection.try_clone()?, client.stream.try_clone()?);

    client.node_stream = Some(proxy_connection);
    client.node_id = Some(node.id);

    if let Err(err) = packet::send_packet(server::Ready::default(), 3, &mut client.stream) {
        return Err(format!("Failed to send ready packet, ({:#?})", err).into());
    };

    println!("Client registered to anticheat server! ({}, {})", client.ip_addr.to_string(), node.id.to_string());

    client.status = ClientStatus::Ready;

    Ok(())
}

pub fn node_client_relay(client_connection: Arc<RwLock<bool>>, mut node_stream: TcpStream, mut client_stream: TcpStream) {
    thread::spawn(move || {
        let mut packet_id_buffer = [0u8; 2];
        let mut data_length_buffer = [0u8; 4];

        loop {
            if let Ok(connection) = client_connection.read() {
                if !*connection {
                    return;
                }
            }

            let packet = match packet::get_packet(&mut node_stream, &mut packet_id_buffer, &mut data_length_buffer) {
                Ok(data) => data,
                Err(err) => {
                    if err.kind() != ConnectionReset {
                        println!("[GOV] [NODE]-[CLIENT] Failed to get packet, ({:#?})", err);
                    }
                    return;
                }
            };

            if packet.id != 10 {
                println!("[GOV] [NODE]-[CLIENT] Packet is not a proxy packet! (ID: {})", packet.id);
                continue;
            }

            if let Err(err) = packet::send_packet(packet.data, 10, &mut client_stream) {
                if err.kind() != ConnectionReset {
                    println!("[GOV] [NODE]-[CLIENT] Failed to send packet to the client, ({:#?})", err);
                }

                return;
            };
        }
    });
}

pub fn disconnect_client(client: &mut Client, reason: DisconnectReason) {
    println!("[GOV] [CLIENT] Client disconnected, Reason: ({:#?})", reason);

    if let Ok(mut connection) = client.connected.write() {
        *connection = false;
    }

    let disconnect_packet = DisconnectServer {
        uuid_key: match client.key {
            Some(key) => Some(key.to_string()),
            None => None
        },
        reason: i32::from(reason),
    };

    let _ = packet::send_packet(disconnect_packet, 2, &mut client.stream);
}