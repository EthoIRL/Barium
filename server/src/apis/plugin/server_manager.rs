use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::anticheat::node_manager::NodeManager;

pub mod server {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    use crate::anticheat::node_manager::NodeManager;
    use crate::apis::plugin::handlers::packet::to_packet;
    use crate::apis::plugin::handlers::play;
    use crate::apis::plugin::handlers::service;

    use std::sync::{Arc, Mutex};

    pub async fn init(mut socket: TcpStream, node_manager: Arc<Mutex<NodeManager>>) {
        let mut buf = [0; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(n) => {
                    if n == 0 {
                        println!("Connection terminated");
                        return;
                    }

                    let received_packet = to_packet(buf, n).await;
                    let mut sending_packet: Option<Vec<u8>> = None;

                    match received_packet.id as i8 {
                        // 0..8 (One through five)
                        // 0 1 2 3 4 5 6 7 8
                        0..=8 => {
                            sending_packet =
                                service::handle_service_packet(received_packet, &node_manager);
                        }
                        // 10..30 (Ten through thirty)
                        10..=30 => {
                            sending_packet = play::handle_play_packet(received_packet, &node_manager);
                        }
                        _ => println!("Unknown packet received! ID: {}", received_packet.id),
                    }

                    if let Some(data) = &sending_packet {
                        if let Err(e) = socket.write_all(&data).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                        }
                    }
                    buf = [0; 1024];
                    drop(sending_packet);
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
        }
    }
}

pub async fn init(node_manager: Arc<Mutex<NodeManager>>, address: &str) {
    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let cloned_node_manager = Arc::clone(&node_manager);
        tokio::spawn(async move {
            server::init(socket, cloned_node_manager).await;
        });
    }
}
