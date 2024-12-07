use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::u16;
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

pub fn get_packet(stream: &mut TcpStream, packet_id_buffer: &mut [u8; 2], data_length_buffer: &mut [u8; 4]) -> Result<(u16, Vec<u8>), Error> {
    stream.read_exact(packet_id_buffer)?;
    let packet_id = u16::from_le_bytes(*packet_id_buffer);

    stream.read_exact(data_length_buffer)?;
    let data_length = u32::from_le_bytes(*data_length_buffer);

    let mut buffer = vec![0u8; data_length as usize];
    stream.read_exact(&mut buffer)?;

    Ok((packet_id, buffer))
}