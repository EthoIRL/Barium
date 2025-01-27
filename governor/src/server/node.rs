use std::{thread, u16};
use std::collections::HashMap;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;
use crate::anticheat::registration::NodeRegistar;
use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::anticheat::{DisconnectNode, NodeResources};
use crate::proto::generic::DisconnectReason;

pub struct Node {
    pub stream: TcpStream,
    pub resources: Arc<Mutex<Option<NodeResources>>>,
    pub connected: Arc<RwLock<bool>>,
    pub id: Uuid
}

pub fn start_node_server(address: (&str, u16), node_list: Arc<RwLock<HashMap<Uuid, Arc<Node>>>>) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(tcp_stream) = stream {
                let peer_address = match tcp_stream.peer_addr() {
                    Ok(addr) => addr.ip(),
                    Err(err) => {
                        eprintln!("[GOV] [NODE] Failed to get peer address: ({err})");
                        continue;
                    }
                };

                println!("[GOV] [NODE] Incoming connection from ({})", peer_address.to_string());

                let node_id = Uuid::new_v4();

                let node = Arc::new(Node {
                    stream: tcp_stream.try_clone().unwrap(),
                    resources: Arc::new(Mutex::new(None)),
                    connected: Arc::new(RwLock::new(true)),
                    id: node_id.clone()
                });

                let node_clone = node.clone();

                match node_list.write() {
                    Ok(mut node_list) => {
                        node_list.insert(node_id, node_clone);
                    },
                    Err(err) => {
                        eprintln!("[GOV] [NODE] Failed to lock node list: ({err})");
                        continue;
                    }
                }

                let cloned_list = node_list.clone();

                thread::spawn(move || handle_node(tcp_stream, node, cloned_list));
            }
        }
    });

    Ok(())
}

pub fn handle_node(mut tcp_stream: TcpStream, mut node: Arc<Node>, node_list: Arc<RwLock<HashMap<Uuid, Arc<Node>>>>) {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    let mut known_packets: HashMap<u16, fn(&mut Arc<Node>, GenericPacket) -> Result<(), Box<dyn std::error::Error>>> = HashMap::new();
    known_packets.insert(0, NodeRegistar::handle);

    loop {
        if let Ok(connection) = node.connected.read() {
            if !*connection {
                return;
            }
        }

        let packet = match packet::get_packet(&mut tcp_stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => {
                println!("[GOV] [NODE] Retrieved data successfully");
                data
            }
            Err(err) => {
                println!("[GOV] Failed to get packet, ({:#?})", err);
                disconnect_node(&mut node, DisconnectReason::Crash);

                if let Ok(mut node_list) = node_list.write() {
                    node_list.remove(&node.id);
                }

                return;
            }
        };

        println!("[GOV] [NODE] PACKET ID: {}", packet.id);

        let packet_handle = match known_packets.get(&packet.id) {
            Some(handler) => handler,
            None => {
                println!("[GOV] [NODE] PacketID not found, ({})", packet.id);
                continue;
            }
        };

        if let Err(err) = packet_handle(&mut node, packet) {
            eprintln!("An error occurred while handling a packet: {err}");

            if let Ok(mut node_list) = node_list.write() {
                node_list.remove(&node.id);
            }

            return;
        };
    }

    unreachable!()
}


pub fn disconnect_node(node: &mut Arc<Node>, reason: DisconnectReason) {
    println!("[GOV] [NODE] Node disconnected, Reason: ({:#?})", reason);

    if let Ok(mut connection) = node.connected.write() {
        *connection = false;
    }

    let disconnect_packet = DisconnectNode {
        reason: i32::from(reason)
    };

    let _ = packet::send_packet(disconnect_packet, 2, &mut node.stream.try_clone().unwrap());
}