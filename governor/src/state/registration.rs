use std::io;

use prost::Message;
use uuid::Uuid;

use crate::API_VERSION;
use crate::proto::{Register, RegistrationResponse};
use crate::state::client::{Client, Status};
use crate::state::packet;

#[derive(Debug)]
pub enum RegistrationError {
    MismatchVersion,
    Unknown,
}

pub fn handle_registration(client: &mut Client, packet: Vec<u8>) -> Result<Register, RegistrationError> {
    let register = match Register::decode(&*packet) {
        Ok(register) => {
            register
        }
        Err(_) => return Err(RegistrationError::Unknown)
    };

    if register.plugin_version != API_VERSION {
        return Err(RegistrationError::MismatchVersion);
    }

    client.state = Some(register);

    Ok(register)
}

pub fn handle_response(client: &mut Client, authenticated: bool) -> Result<(), io::Error> {
    let mut response = RegistrationResponse {
        succeeded: authenticated,
        uuid_key: None,
    };

    let key = Uuid::new_v4();
    client.key = Some(key);
    response.uuid_key = Some(key.to_string());

    packet::send_packet(response, 1, &mut client.stream)?;

    Ok(())
}