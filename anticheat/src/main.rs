use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use structopt::StructOpt;
use uuid::Uuid;
use crate::client::state::game::GameServer;
use crate::node::{auth, connection};

mod node;
mod packet;
mod proto;
mod client;

pub const API_VERSION: i32 = 0;

#[derive(StructOpt, Debug)]
#[structopt(name = "Barium Anticheat - Etho", no_version)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
#[structopt(setting = structopt::clap::AppSettings::DisableVersion)]
pub struct GenericArguments {
    /// Governor's remote ip address
    #[structopt(long = "address", default_value = "127.0.0.1")]
    pub governor_address: String,

    /// Governor's remote port
    #[structopt(long = "port", default_value = "3349")]
    pub governor_port: u16,

    /// Governor's exchange authentication key
    #[structopt(long = "key", default_value = "shared key", env = "GOVERNOR_KEY")]
    pub governor_key: String
}

fn main() {
    let arguments: GenericArguments = GenericArguments::from_args();
    println!("[NODE] Connecting to governor... ({}:{}) (v{})", arguments.governor_address, arguments.governor_port, API_VERSION);

    let mut stream = match TcpStream::connect((arguments.governor_address.clone(), arguments.governor_port)) {
        Ok(stream) => {
            println!("[NODE] Successfully connected to remote governor");
            stream
        },
        Err(err) => {
            println!("[NODE] Failed to connect to governor server... ({})", err);
            return;
        }
    };

    if let Err(err) = auth::authed_connection(&mut stream, &arguments.governor_key) {
        println!("[NODE] Failed to send authed registration packet... ({})", err);
        return;
    }

    if let Err(err) = connection::handle_registration(&mut stream) {
        println!("[NODE] {}", err);
        return;
    };

    let servers: Arc<RwLock<HashMap<Uuid, Arc<GameServer>>>> = Arc::new(RwLock::new(HashMap::new()));

    connection::start_client_server((arguments.governor_address, arguments.governor_port), &mut stream, servers.clone());
}
