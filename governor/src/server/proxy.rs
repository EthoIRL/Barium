use std::{thread, u16};
use std::io::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::state::client;
use crate::state::client::{Client, Status};

pub fn start_proxy_server(address: (&str, u16)) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>, Error> {
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
                    pool.push(thread::spawn(|| client::handle_client(client)));
                }
            }
        }
    }); 

    Ok(thread_pool)
}