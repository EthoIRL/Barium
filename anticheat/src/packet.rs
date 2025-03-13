use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::u16;
use prost::Message;
use crate::proto::generic::ProxyMessage;

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

pub fn send_proxied_packet(packet: impl Message, packet_id: u16, stream: &mut TcpStream) -> Result<(), Error> {
    let original_data_buffer: Vec<u8> = packet.encode_to_vec();

    let proxied_message = ProxyMessage {
        message_id: packet_id as u32,
        message_data: encoding_rs::mem::decode_latin1(&*original_data_buffer).to_string()
    };

    let data_buffer: Vec<u8> = proxied_message.encode_to_vec();

    let packet_id: [u8; 2] = u16::to_le_bytes(10);
    let data_length: [u8; 4] = u32::to_le_bytes(data_buffer.len() as u32);

    stream.write_all(&packet_id)?;
    stream.write_all(&data_length)?;
    stream.write_all(&data_buffer)?;
    stream.flush()?;

    Ok(())
}


pub struct GenericPacket {
    pub id: u16,
    pub data: Vec<u8>
}

impl GenericPacket {
    pub fn decode<T: Message + Default>(&self) -> Result<T, Error> {
        T::decode(&*self.data).map_err(|err| Error::from(err))
    }
}

pub trait GenericHandler<T, Y> {
    fn handle(parent: &mut T, packet: Y) -> Result<(), Box<dyn std::error::Error>>;
    fn id() -> u16;
}

pub fn get_packet(stream: &mut TcpStream, packet_id_buffer: &mut [u8; 2], data_length_buffer: &mut [u8; 4]) -> Result<GenericPacket, Error> {
    stream.read_exact(packet_id_buffer)?;
    let packet_id = u16::from_le_bytes(*packet_id_buffer);

    stream.read_exact(data_length_buffer)?;
    let data_length = u32::from_le_bytes(*data_length_buffer);

    let mut buffer = vec![0u8; data_length as usize];
    stream.read_exact(&mut buffer)?;

    Ok(GenericPacket {
        id: packet_id,
        data: buffer
    })
}