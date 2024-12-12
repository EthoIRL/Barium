mod node;
mod packet;
mod proto;

pub const API_VERSION: i32 = 0;

const GOVERNOR: (&str, u16) = ("127.0.0.1", 3238);

fn main() {
    println!("Hello, world!");
}
