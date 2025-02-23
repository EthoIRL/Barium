use crate::packet;
use crate::packet::{GenericHandler, GenericPacket};
use crate::proto::server::ProxyMessage;
use crate::server::client::Client;

pub struct ClientProxy;

impl GenericHandler<Client, GenericPacket> for ClientProxy {
    fn handle(client: &mut Client, packet: GenericPacket) -> Result<(), Box<dyn std::error::Error>> {
        let proxy_message = packet.decode::<ProxyMessage>()?;

        if let Some(node_stream) = client.node_stream.as_mut() {
            packet::send_raw(&*encoding_rs::mem::encode_latin1_lossy(&proxy_message.message_data), proxy_message.message_id as u16, node_stream)?;

            return Ok(());
        }

        return Err("Node stream cannot be found and referenced!".into());
    }
}