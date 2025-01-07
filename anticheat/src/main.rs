use std::thread;
use crate::node::mesh;

mod node;
mod packet;
mod proto;

pub const API_VERSION: i32 = 0;

const NODE_GOVERNOR: (&str, u16) = ("127.0.0.1", 3349);

fn main() {
    println!("Hello, world!");

    for i in 0..2 {
        thread::spawn(|| mesh::authed_connection(NODE_GOVERNOR, "shared key"));
    }

    loop {}
}
