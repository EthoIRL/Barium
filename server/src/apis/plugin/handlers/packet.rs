use serde::Serialize;
use tokio::io::AsyncReadExt;

pub struct Packet {
    pub id: u8,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(id: u8, data: Vec<u8>) -> Self {
        Packet { id: id, data: data }
    }
}

pub async fn to_packet(buffer: [u8; 1024], read_size: usize) -> Packet {
    let mut byte_identifier: &[u8] = &[buffer[0]];
    let id = AsyncReadExt::read_u8(&mut byte_identifier).await.unwrap();
    let data = buffer[1..read_size].to_vec();

    Packet { id: id, data: data }
}

pub fn to_string(packet: Packet) -> Option<String> {
    let decoded_json_string = match String::from_utf8(packet.data) {
        Ok(string) => string,
        Err(_) => return None,
    };

    Some(decoded_json_string.trim_matches(char::from(0)).to_string())
}

pub fn to_buffer<T: Serialize>(id: [u8; 1], packet_data: T) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend(id);
    data.extend(serde_json::to_vec(&packet_data).unwrap());
    data
}
