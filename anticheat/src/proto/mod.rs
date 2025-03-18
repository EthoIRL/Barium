pub mod generic {
    include!(concat!(env!("OUT_DIR"), "/generic.rs"));
}

pub mod anticheat {
    include!(concat!(env!("OUT_DIR"), "/anticheat.rs"));
}

pub mod game {
    include!(concat!(env!("OUT_DIR"), "/network.rs"));
    include!(concat!(env!("OUT_DIR"), "/movement.rs"));
    include!(concat!(env!("OUT_DIR"), "/command.rs"));
    include!(concat!(env!("OUT_DIR"), "/player.rs"));
    include!(concat!(env!("OUT_DIR"), "/collision.rs"));
}