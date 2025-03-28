# Barium
MCJE 1.8 - 1.21 Remote anticheat solution written in Rust, for the fun of it :)

## Architecture
![image](https://github.com/user-attachments/assets/b51ba19a-cb5c-4e7b-8876-4aa73d6a3690)

## V1 vs V2
Uses a protobuf based communication protocol rather than a hand rolled layer. <br>
See [PACKETS.md](PACKETS.md) for general proto id & descriptors for each packet.

## Dependencies
* Protoc - v29.3
* Rust (Msrv 1.80)
* Java 21 (Maven)

## TODO
- [X] Modularity & Scalability
  - Anticheat - Governor - Plugin 
  - Multi anticheat, Single Governor, Multi Plugin
- [X] Cross module communication
- [X] Reverse proxy communication
- [ ] Connection load balancing
- [x] Multithreading by default
- [ ] Secure authentication
- [ ] Secure communications
- [X] Anticheat Checks
- [X] Player warning
- [X] Connection fully fail-safe
- [X] Windows/Linux cross compilation
- [ ] Command line arguments