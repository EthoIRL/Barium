use std::io;
use std::io::Read;
use prost::Message;
use uuid::Uuid;
use crate::API_VERSION;
use crate::proto::{Register, RegistrationResponse};
use crate::state::client::{Client, MAXIMUM_PACKET_SIZE};
use crate::state::packet::send_packet;

#[derive(Debug)]
pub enum RegistrationError {
    SizeOverload,
    MismatchVersion,
    InvalidId,
    Unknown,
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
                    }
                    Err(_) => Err(RegistrationError::Unknown)
                };
            }
        }
    }

    Err(RegistrationError::Unknown)
}

pub fn handle_response(client: &mut Client, authenticated: bool) -> Result<(), io::Error> {
    let mut response = RegistrationResponse {
        succeeded: authenticated,
        uuid_key: None,
    };

    let key = Uuid::new_v4();
    client.key = Some(key);
    response.uuid_key = Some(key.to_string());

    send_packet(response, 1, &mut client.stream)?;

    Ok(())
}