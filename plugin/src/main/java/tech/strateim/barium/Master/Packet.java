package tech.strateim.barium.Master;

import com.google.protobuf.GeneratedMessageV3;

import java.io.IOException;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.logging.Logger;

public class Packet {
    public static void SendPacket(GeneratedMessageV3 packet, int id, OutputStream stream, Logger log) {
        byte[] data = packet.toByteArray();
        byte[] dataLength = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN).putInt(data.length).array();
        byte[] packetId = ByteBuffer.allocate(2).order(ByteOrder.LITTLE_ENDIAN).putShort((short)id).array();

        try {
            stream.write(dataLength);
            stream.write(packetId);
            stream.write(data);
            stream.flush();
        } catch (Exception ex) {
            try {
                Thread.sleep(1000);
            } catch (Exception ignored) {}

            log.warning("Packet failed to send retrying... [" + id + "]");

            SendPacketRetry(packet, id, stream, log, (byte) 0);
        }
    }


    private static void SendPacketRetry(GeneratedMessageV3 packet, int id, OutputStream stream, Logger log, byte recursed) {
        byte[] data = packet.toByteArray();
        byte[] dataLength = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN).putInt(data.length).array();
        byte[] packetId = ByteBuffer.allocate(2).order(ByteOrder.LITTLE_ENDIAN).putShort((short)id).array();

        try {
            stream.write(dataLength);
            stream.write(packetId);
            stream.write(data);
            stream.flush();
        } catch (Exception ex) {
            try {
                Thread.sleep(1000);
            } catch (Exception ignored) {}

            recursed++;

            if (recursed > 5) {
                log.severe("Packet failed fully failed to send. ");
                return;
            }
            log.warning("Packet failed to send retrying... [" + id + "]");

            SendPacketRetry(packet, id, stream, log, recursed);
        }
    }
}
