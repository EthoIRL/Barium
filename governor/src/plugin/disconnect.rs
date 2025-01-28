use std::str::FromStr;
use uuid::Uuid;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::server::DisconnectServer;
use crate::server::client;
use crate::server::client::{Client, ClientStatus};

pub struct ClientDisconnect;

impl GenericHandler<Client, GenericPacket> for ClientDisconnect {
    fn handle(client: &mut Client, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let disconnect_packet = packet.decode::<DisconnectServer>()?;
        
        if client.status == ClientStatus::Initialization {
            client::disconnect_client(client, disconnect_packet.reason());
            return Ok(());
        }

        assert!(&disconnect_packet.uuid_key.is_some());
        assert!(client.key.is_some(), "Server key invalid past Initialization phase during disconnection.");

        let packet_uuid = Uuid::from_str(&disconnect_packet.uuid_key())?;

        let server_uuid = client.key.unwrap();

        if server_uuid != packet_uuid {
            return Err(format!("Server uuid does not match received packet uuid, (Server: {}, Packet: {})", server_uuid, packet_uuid).into());
        }

        client::disconnect_client(client, disconnect_packet.reason());
        return Ok(())
    }
}