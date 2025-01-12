use std::io;
use std::error::Error;
use prost::Message;
use uuid::Uuid;

use crate::API_VERSION;
use crate::error::RegistrationError;
use crate::proto::server::server_registration::{Register, Response};
use crate::server::client::{Client, Status};
use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::generic::DisconnectReason;
use crate::server::client;

pub fn verify_registration(packet: Vec<u8>) -> Result<Register, RegistrationError> {
    let register = match Register::decode(&*packet) {
        Ok(register) => {
            register
        }
        Err(_) => return Err(RegistrationError::Unknown)
    };

    if register.plugin_version != API_VERSION {
        return Err(RegistrationError::MismatchVersion {
            supplied_version: register.plugin_version
        });
    }

    Ok(register)
}

fn handle_response(client: &mut Client, authenticated: bool) -> Result<Uuid, io::Error> {
    let mut response = Response {
        succeeded: authenticated,
        uuid_key: None,
    };

    let key = Uuid::new_v4();
    response.uuid_key = Some(key.to_string());

    packet::send_packet(response, 1, &mut client.stream)?;

    Ok(key)
}

pub struct ClientRegistration;

impl GenericHandler<Client, GenericPacket> for ClientRegistration {
    fn handle(client: &mut Client, packet: GenericPacket) -> Result<(), Box<dyn Error>> {
        let result = verify_registration(packet.data);

        let key  = match handle_response(client, result.is_ok()) {
            Ok(key) => key,
            Err(_) => return Err(RegistrationError::BadResponse.into())
        };

        return match result {
            Ok(register) => {
                client.state = Some(register);
                client.status = Status::Registered;
                client.key = Some(key);

                println!("{:#?}", register);
                Ok(())
            },
            Err(err) => {
                println!("Failed to authenticate client, (Reason: {:#?})", err);

                client::disconnect_client(client, match err {
                    RegistrationError::MismatchVersion {supplied_version} => DisconnectReason::MismatchVersion,
                    _ => DisconnectReason::Unknown
                });

                Err(err.into())
            }
        }
    }
}