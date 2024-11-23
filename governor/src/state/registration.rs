use std::io::{Read, Write};
use prost::Message;
use uuid::Uuid;
use crate::API_VERSION;
use crate::proto::{Register, RegistrationResponse};
use crate::state::client::{Client, MAXIMUM_PACKET_SIZE, Status};

#[derive(Debug)]
pub enum RegistrationError {
    SizeOverload,
    MismatchVersion,
    InvalidId,
    Unknown
}

pub fn handle_registration(client: &mut Client, data_length_buffer: &mut [u8; 4]) -> Result<Register, RegistrationError> {
    if client.stream.read_exact(data_length_buffer).is_ok() {
        let data_length = u32::from_le_bytes(*data_length_buffer);

        let mut packet_id_buffer = [0u8; 2];
        if client.stream.read_exact(&mut packet_id_buffer).is_ok() {
            let packet_id = u16::from_le_bytes(packet_id_buffer);

            if data_length > MAXIMUM_PACKET_SIZE {
                return Err(RegistrationError::SizeOverload);
            }

            if packet_id != 0 {
                return Err(RegistrationError::InvalidId);
            }

            let mut buffer = vec![0u8; data_length as usize];
            if client.stream.read_exact(&mut buffer).is_ok() {
                return match Register::decode(&*buffer) {
                    Ok(register) => {
                        if register.plugin_version != API_VERSION {
                            return Err(RegistrationError::MismatchVersion);
                        } 
                        
                        Ok(register)
                    },
                    Err(_) => Err(RegistrationError::Unknown)
                }
            }
        }
    }

    Err(RegistrationError::Unknown)
}

pub fn handle_response(client: &mut Client) {
    let mut response = RegistrationResponse {
        succeeded: client.status == Status::Registered,
        uuid_key: None
    };
    
    let key = Uuid::new_v4();
    client.key = Some(key);
    response.uuid_key = Some(key.to_string());
    
    let data_buffer: Vec<u8> = response.encode_to_vec();

    let packet_id: [u8; 2] = u16::to_le_bytes(1);
    let data_length: [u8; 4] = u32::to_le_bytes(data_buffer.len() as u32);

    client.stream.write_all(&packet_id).unwrap();
    client.stream.write_all(&data_length).unwrap();
    client.stream.write_all(&data_buffer).unwrap();
    client.stream.flush().unwrap();
}