use std::{thread, u16, u32};
use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use threadpool::ThreadPool;
use crate::state::client;
use crate::state::client::{Client, Status};

pub fn start_proxy_server(address: (&str, u16)) -> Result<Arc<ThreadPool>, Error> {
    let listener = TcpListener::bind(address).unwrap();

    // TODO: Create unit tests for tons of clients

    let thread_pool = Arc::new(ThreadPool::new(8));
    let pool = thread_pool.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(tcp_stream) = stream {
                println!("[GOV] Incoming connection");

                let client = Client {
                    stream: tcp_stream,
                    status: Status::Init,
                    state: None
                };

                pool.execute(|| client::handle_client(client));
            }
        }
    }); 

    Ok(thread_pool)
}