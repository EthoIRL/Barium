use std::net::TcpStream;
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
        thread::spawn(|| {
            let mut stream = TcpStream::connect(NODE_GOVERNOR).unwrap();
            mesh::authed_connection(&mut stream, "shared key").unwrap();
            println!("Fully authenticated!!");
        });
    }

    loop {}
}
