use std::{thread, u16};
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

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
    loop {

    }
}