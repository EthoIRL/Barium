# Barium
**Proof of concept** for a remote minecraft 1.8.X-1.19.X anti-cheat with all checks and calculations handled on a remote server.

## Setup
- For the Minecraft plugin make sure to change the maven build output directory in the pom.
- Ip's & ports are currently hardcoded so you might need to change them if you aren't using planning on using localhost.

# Built on:
- Backend:
    * Rust 1.67
    * Tokio (Custom TCP API)
- Plugin:
    * Java 17 (Maven)
    * PacketEvents