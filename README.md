# Barium
MCJE 1.8 - 1.21 remote based anti cheat solution based in Rust

## Architecture
![image](https://github.com/user-attachments/assets/b51ba19a-cb5c-4e7b-8876-4aa73d6a3690)

## Stack
#### MCJE Plugin:
- Java 21 (MAVEN)
- PacketEvents

#### Governor:
- Rust (MSRV 1.80)
- Prost

## V1 vs V2
Uses a protobuf based communication protocol rather than a hand rolled layer. <br>
See [PACKETS.md](PACKETS.md) for general proto id & descriptors for each packet.

## Dependencies
* Protoc - v29.3