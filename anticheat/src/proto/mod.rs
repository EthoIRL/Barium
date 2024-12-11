pub mod generic {
    include!(concat!(env!("OUT_DIR"), "/generic.rs"));
}

pub mod anticheat {
    include!(concat!(env!("OUT_DIR"), "/anticheat.rs"));
}

pub mod server {
    include!(concat!(env!("OUT_DIR"), "/server.rs"));
}