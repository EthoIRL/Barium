use strum::FromRepr;
use crate::API_VERSION;

const GENERIC_HEADER_OFFSET: usize = 3;

#[derive(Debug, Clone)]
enum MalformedData {
    InvalidLength,
    MismatchVersion,
    UnknownId
}

#[derive(FromRepr, Debug, Clone, PartialEq)]
#[repr(u8)]
enum KnownPackets {
    Register = 0,
}

pub fn handle_generic_packet(data: Vec<u8>) -> Result<(), MalformedData> {
    if data.is_empty() || data.len() < 2 {
        return Err(MalformedData::InvalidLength);
    }

    let version_data: [u8; 2] = {
        [data.get(0).unwrap().clone(), data.get(1).unwrap().clone()]
    };

    let version = u16::from_le_bytes(version_data);

    // if API_VERSION != version {
    //     return Err(MalformedData::MismatchVersion);
    // }

    let id: u8 = data.get(0).unwrap().clone();

    let packet_enum = match KnownPackets::from_repr(id) {
        Some(packet) => packet,
        None => return Err(MalformedData::UnknownId)
    };

    match packet_enum {
        KnownPackets::Register => handle_registration_packet(data)
    }

    Ok(())
}

fn handle_registration_packet(data: Vec<u8>) {
    let os_ver = data.get(4).unwrap();
    let via = data.get(5).unwrap();
    let ip = &data[5..=9];
}


// const GENERIC_HEADER_OFFSET: usize = 3;
// 
// #[derive(Debug, Clone)]
// enum MalformedData {
//     InvalidLength,
//     MismatchVersion,
//     UnknownId
// }
// 
// #[derive(FromRepr, Debug, Clone, PartialEq)]
// #[repr(u8)]
// enum KnownPackets {
//     Register = 0,
// }
// 
// pub fn handle_generic_packet(data: Vec<u8>) -> Result<(), MalformedData> {
//     if data.is_empty() || data.len() < 2 {
//         return Err(MalformedData::InvalidLength);
//     }
// 
//     let version_data: [u8; 2] = {
//         [data.get(0).unwrap().clone(), data.get(1).unwrap().clone()]
//     };
// 
//     let version = u16::from_le_bytes(version_data);
// 
//     if API_VERSION != version {
//         return Err(MalformedData::MismatchVersion);
//     }
// 
//     let id: u8 = data.get(0).unwrap().clone();
// 
//     let packet_enum = match KnownPackets::from_repr(id) {
//         Some(packet) => packet,
//         None => return Err(MalformedData::UnknownId)
//     };
// 
//     match packet_enum {
//         KnownPackets::Register => handle_registration_packet(data)
//     }
// 
//     Ok(())
// }
// 
// fn handle_registration_packet(data: Vec<u8>) {
//     let os_ver = data.get(4).unwrap();
//     let via = data.get(5).unwrap();
//     let ip = &data[5..=9];
// }