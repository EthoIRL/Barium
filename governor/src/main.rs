use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use uuid::Uuid;

use crate::server::{client, node};
use crate::server::node::Node;

mod server;
mod proto;
mod plugin;
pub mod packet;
mod anticheat;
mod error;

pub const API_VERSION: i32 = 0;

const NODE_KEY: &str = "shared key";

#[derive(StructOpt, Debug)]
#[structopt(name = "Barium Governor - Etho", no_version)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
#[structopt(setting = structopt::clap::AppSettings::DisableVersion)]
pub struct GenericArguments {
    #[structopt(long = "clients", default_value = "127.0.0.1:3238")]
    pub remote_client_address: String,

    #[structopt(long = "nodes", default_value = "127.0.0.1:3349")]
    pub remote_node_address: String,

    #[structopt(long = "key", default_value = "shared key", env = "GOVERNOR_KEY")]
    pub node_key: String
}

fn main() {
    let arguments: GenericArguments = GenericArguments::from_args();

    let client_arguments= arguments.remote_client_address.split(":").collect::<Vec<_>>();
    let client_address = (*client_arguments.first().unwrap(), u16::from_str(*client_arguments.last().unwrap()).unwrap());

    let node_arguments = arguments.remote_node_address.split(":").collect::<Vec<_>>();
    let node_address = (*node_arguments.first().unwrap(), u16::from_str(*node_arguments.last().unwrap()).unwrap());

    println!("[GOV] Starting proxy server, {:?}", &client_address);
    println!("[GOV] Starting node server, {:?}", &node_address);

    let nodes: Arc<RwLock<HashMap<Uuid, Arc<Node>>>> = Arc::new(RwLock::new(HashMap::new()));

    if let Err(err) = node::start_node_server(node_address, arguments.node_key, nodes.clone()) {
        eprintln!("[GOV] Failed to start node server, ({})", err);
        return;
    }

    if let Err(err) = client::start_plugin_server(client_address, nodes.clone()) {
        eprintln!("[GOV] Failed to start client server, ({})", err);
        return;
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
