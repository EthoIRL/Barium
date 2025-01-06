use std::{thread, u16};
use std::collections::HashMap;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::anticheat::registration::NodeRegistar;
use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::anticheat::DisconnectNode;
use crate::proto::generic::DisconnectReason;

pub struct Node {
    pub stream: TcpStream,
}

pub fn start_node_server(address: (&str, u16), node_list: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;

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

                let node = Arc::new(Mutex::new(Node {
                    stream: tcp_stream
                }));

                if let Ok(mut node_list) = node_list.lock() {
                    node_list.push(node.clone());
                }

                thread::spawn(|| handle_node(node));
            }
        }
    });

    Ok(())
}

pub fn handle_node(mut node: Arc<Mutex<Node>>) {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    let mut known_packets: HashMap<u16, fn(&mut Node, GenericPacket) -> Result<(), Box<dyn std::error::Error>>> = HashMap::new();
    known_packets.insert(8, NodeRegistar::handle);

    loop {
        if let Ok(mut node) = node.lock() {
            let packet = match packet::get_packet(&mut node.stream, &mut packet_id_buffer, &mut data_length_buffer) {
                Ok(data) =>  {
                    println!("Retrieved data successfully");
                    data
                },
                Err(err) => {
                    println!("[GOV] Failed to get packet, ({:#?})", err);
                    return;
                }
            };

            println!("PACKET ID: {}", packet.id);

            let packet_handler = known_packets.get(&packet.id).unwrap();
            packet_handler(&mut node, packet).unwrap();
        };
    }
}


pub fn disconnect_node(node: &mut Node, reason: DisconnectReason) {
    let disconnect_packet = DisconnectNode {
        reason: i32::from(reason)
    };

    let _ = packet::send_packet(disconnect_packet, 2, &mut node.stream);
}