use std::net::TcpStream;
use rand::thread_rng;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use crate::{API_VERSION, packet};
use crate::proto::anticheat::{node_registration, NodeResources};
use crate::proto::anticheat::node_registration::EncryptionSync;

pub fn authed_connection(stream: &mut TcpStream, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let packet = packet::get_packet(stream, &mut [0u8; 2], &mut [0u8; 4])?;
    if packet.id != 4 {
        return Err("[NODE] Packet received during auth connection state is not encryption sync packet.".into());
    }

    let mut rng = thread_rng();

    let encryption_sync = packet.decode::<EncryptionSync>()?;

    let public_key = RsaPublicKey::from_public_key_pem(&encryption_sync.public_key)?;

    let encrypted_key = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, key.as_bytes())?;

    let registration = node_registration::Register {
        node_version: API_VERSION,
        shared_key: encoding_rs::mem::decode_latin1(&encrypted_key).to_string(),
        resources: Some(NodeResources {
            memory: 1,
            cpu: 1,
        }),
    };

    packet::send_packet(registration, 0, stream).map_err(|error| error.into())
}