use std::{thread, u16};
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct Node {
    pub stream: TcpStream,
}

pub fn start_node_server(address: (&str, u16)) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>, Error> {
    let thread_pool: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));

    let pool = thread_pool.clone();
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

                let node = Node {
                    stream: tcp_stream
                };

                if let Ok(mut pool) = pool.lock() {
                    pool.push(thread::spawn(|| handle_client(node)));
                }
            }
        }
    });

    Ok(thread_pool)
}

pub fn handle_client(mut node: Node) {

}