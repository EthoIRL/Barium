use std::io::{Error, Write};
use std::net::TcpStream;
use prost::Message;

pub fn send_packet(packet: impl Message, packet_id: u16, stream: &mut TcpStream) -> Result<(), Error> {
    let data_buffer: Vec<u8> = packet.encode_to_vec();

    let packet_id: [u8; 2] = u16::to_le_bytes(packet_id);
    let data_length: [u8; 4] = u32::to_le_bytes(data_buffer.len() as u32);

    stream.write_all(&packet_id)?;
    stream.write_all(&data_length)?;
    stream.write_all(&data_buffer)?;
    stream.flush()?;
    
    Ok(())
}