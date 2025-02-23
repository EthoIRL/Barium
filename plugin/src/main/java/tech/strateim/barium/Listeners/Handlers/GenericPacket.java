package tech.strateim.barium.Listeners.Handlers;

import com.google.protobuf.Message;

import javax.annotation.Nullable;

@Nullable
public class GenericPacket {
    private final int packetId;
    private final Message packetData;

    public GenericPacket(Message PacketData, int PacketId) {
        packetData = PacketData;
        packetId = PacketId;
    }

    public int getPacketId() {
        return packetId;
    }

    public Message getPacketData() {
        return packetData;
    }
}
