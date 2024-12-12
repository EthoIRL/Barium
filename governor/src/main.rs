use std::thread;
use std::time::Duration;

use crate::server::plugin;
use crate::server::plugin::Client;

mod server;
mod proto;
mod state;

pub const API_VERSION: i32 = 0;

static mut CLIENTS: Vec<Client> = Vec::new();

fn main() {
    let address = ("127.0.0.1", 3238);
    println!("[GOV] Starting proxy server, {:?}", &address);

    plugin::start_plugin_server(address).unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}