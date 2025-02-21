package tech.strateim.barium.Listeners.Handlers;

import com.google.protobuf.GeneratedMessageV3;

import javax.annotation.Nullable;

@Nullable
public class GenericPacket {
    private final int packetId;
    private final GeneratedMessageV3 packetData;

    public GenericPacket(GeneratedMessageV3 PacketData, int PacketId) {
        packetData = PacketData;
        packetId = PacketId;
    }

    public int getPacketId() {
        return packetId;
    }

    public GeneratedMessageV3 getPacketData() {
        return packetData;
    }
}
