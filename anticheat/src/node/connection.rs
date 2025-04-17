use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;
use prost::Message;
use uuid::Uuid;
use crate::client::checks::check::GenericCheck;
use crate::client::checks::movement::fly::y_prediction::YPrediction;
use crate::client::checks::player::bad_packets::illegal_flying::IllegalFlying;
use crate::client::state::game::GameServer;

use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::anticheat::node_registration::Response;
use crate::proto::anticheat::NodeProxyNegotiation;
use crate::proto::game::{PxPlayerClientAbilities, PxPlayerCollision, PxPlayerJoin, PxPlayerLeave, PxPlayerMovement, PxPlayerRotation, PxServerTick};

pub fn handle_registration(stream: &mut TcpStream) -> Result<(), Error> {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    loop {
        let packet = packet::get_packet(stream, &mut packet_id_buffer, &mut data_length_buffer)?;

        assert_eq!(packet.id, 1, "Unknown packet received during registration response phase");

        if packet.id != 1 {
            return Err(Error::new(ErrorKind::Other, format!("Unknown packet received during registration response phase, ({})", packet.id)));
        }

        let response = Response::decode(&*packet.data)?;

        if response.succeeded {
            return Ok(());
        }

        return Err(Error::new(ErrorKind::Other, "Failed to authenticate with governor server!"));
    }

    unreachable!()
}

pub fn start_client_server(governor_address: (String, u16), stream: &mut TcpStream, game_servers: Arc<RwLock<HashMap<Uuid, Arc<GameServer>>>>) {
    let mut packet_id_buffer = [0u8; 2];
    let mut data_length_buffer = [0u8; 4];

    let mut packet_handles: HashMap<u16, fn(&mut Arc<GameServer>, GenericPacket) -> Result<(), Box<dyn std::error::Error>>> = HashMap::new();
    packet_handles.insert(PxServerTick::id(), PxServerTick::handle);
    packet_handles.insert(PxPlayerJoin::id(), PxPlayerJoin::handle);
    packet_handles.insert(PxPlayerLeave::id(), PxPlayerLeave::handle);
    packet_handles.insert(PxPlayerMovement::id(), PxPlayerMovement::handle);
    packet_handles.insert(PxPlayerRotation::id(), PxPlayerRotation::handle);
    packet_handles.insert(PxPlayerClientAbilities::id(), PxPlayerClientAbilities::handle);
    packet_handles.insert(PxPlayerCollision::id(), PxPlayerCollision::handle);

    let arc_packet_handles = Arc::new(packet_handles);

    loop {
        let packet = match packet::get_packet(stream, &mut packet_id_buffer, &mut data_length_buffer) {
            Ok(data) => data,
            Err(err) => {
                println!("[NODE] [GOVERNOR] Failed to retrieve packet from governor. ({err})");
                return;
            }
        };

        if packet.id != 3 {
            println!("[NODE] [GOVERNOR] Unknown packet received (ID: {})", packet.id);
            continue;
        }

        let negotiation = match NodeProxyNegotiation::decode(&*packet.data) {
            Ok(packet) => packet,
            Err(err) => {
                println!("[NODE] [GOVERNOR] Failed to decode packet. ({err})");
                continue;
            }
        };

        let game_server_key = match Uuid::from_str(&negotiation.client_key) {
            Ok(key) => key,
            Err(err) => {
                println!("[NODE] [GOVERNOR] Unable to decode uuid from key. ({err})");
                continue;
            }
        };

        let game_servers = game_servers.clone();
        let remote_address = governor_address.0.to_string();
        let packets_handles = arc_packet_handles.clone();

        thread::spawn(move || {
            let mut connection = match TcpStream::connect((remote_address.as_str(), negotiation.port as u16)) {
                Ok(connection) => connection,
                Err(err) => {
                    println!("[NODE] [NODE]-[GOVERNOR] Couldn't connect to dynamic governor listener ({err})");
                    return;
                }
            };

            let mut game_server = Arc::new(GameServer {
                relay: connection.try_clone().unwrap(),
                key: game_server_key.clone(),
                info: negotiation.server_info.unwrap(),
                players: RwLock::new(HashMap::new())
            });

            match game_servers.write() {
                Ok(mut servers) => {
                    println!("[NODE] Registered new game server ({})", game_server_key.to_string());
                    servers.insert(game_server_key, game_server.clone());
                },
                Err(err) => {
                    println!("[NODE] Failed to write to in memory game server list. ({})", err);
                    return;
                }
            }

            let mut packet_id_buffer = [0u8; 2];
            let mut data_length_buffer = [0u8; 4];

            loop {
                let proxy_packet = match packet::get_packet(&mut connection, &mut packet_id_buffer, &mut data_length_buffer) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("[NODE] [GOVERNOR]-[CLIENT] Unknown packet received ({err})");
                        break;
                    }
                };

                println!("[NODE] [GOVERNOR]-[CLIENT] Received from proxy connection (Id: {})", proxy_packet.id);

                if proxy_packet.id == 0 {
                    println!("[NODE] [GOVERNOR]-[CLIENT] Game server connection has been shutdown");
                    if let Ok(mut game_servers) = game_servers.write() {
                        game_servers.remove(&game_server_key);
                    }
                    break;
                }

                let packet_handle = match packets_handles.get(&proxy_packet.id) {
                    Some(handler) => handler,
                    None => {
                        println!("[NODE] Unknown PacketID found from Client, ({})", proxy_packet.id);
                        continue;
                    }
                };

                if let Err(err) = packet_handle(&mut game_server, proxy_packet) {
                    println!("[NODE] [CLIENT] An error occurred while handling a proxied game packet, ({})", err);
                    continue;
                };
            }
        });
    }
}