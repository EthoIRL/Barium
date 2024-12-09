use std::thread;
use std::time::Duration;

use crate::server::proxy;
use crate::state::client::Client;

mod server;
mod proto;
mod state;

pub const API_VERSION: i32 = 0;

static mut CLIENTS: Vec<Client> = Vec::new();

fn main() {
    let address = ("127.0.0.1", 3238);
    println!("[GOV] Starting proxy server, {:?}", &address);

    proxy::start_proxy_server(address).unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}