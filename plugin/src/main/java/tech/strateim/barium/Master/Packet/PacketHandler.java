package tech.strateim.barium.Master.Packet;

import com.google.protobuf.GeneratedMessageV3;
import org.jetbrains.annotations.Nullable;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;
import java.util.logging.Logger;

public class PacketHandler {
    private final Logger Log;
    private final OutputStream SendSocket;
    private final Lock SendLock;
    private final InputStream ReceiveSocket;
    private final byte MAX_RETRIES = 4;
    private final int PACKET_WAIT = 500;

    public PacketHandler(Logger log, OutputStream sender, InputStream receiver) {
        Log = log;
        SendSocket = sender;
        ReceiveSocket = receiver;

        SendLock = new ReentrantLock();
    }

    public void SendPacket(GeneratedMessageV3 packet, int id) {
        byte[] data = packet.toByteArray();
        byte[] dataLength = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN).putInt(data.length).array();
        byte[] packetId = ByteBuffer.allocate(2).order(ByteOrder.LITTLE_ENDIAN).putShort((short)id).array();

        try {
            WritePacketBlocking(dataLength, packetId, data);
        } catch (Exception ex) {
            try {
                Thread.sleep(1000);
            } catch (Exception ignored) {}

            SendPacketRetry(dataLength, packetId, data);
        }
    }


    private void SendPacketRetry(byte[] dataLength, byte[] packetId, byte[] data) {
        byte recursed = 0;

        while(true) {
            try {
                WritePacketBlocking(dataLength, packetId, data);
            } catch (Exception ex) {
                try {
                    Thread.sleep(PACKET_WAIT);
                } catch (Exception ignored) {}

                recursed++;

                if (recursed == MAX_RETRIES) {
                    Log.severe("Packet failed fully failed to send.");
                    Log.severe(ex.toString());
                    return;
                }

                continue;
            }

            return;
        }
    }

    private void WritePacketBlocking(byte[] dataLength, byte[] packetId, byte[] data) throws IOException {
        try {
            SendLock.lock();
            SendSocket.write(dataLength);
            SendSocket.write(packetId);
            SendSocket.write(data);
            SendSocket.flush();
        } finally {
            SendLock.unlock();
        }
    }

    public @Nullable Packet ReceivePacketBlocking() throws Exception {
        ByteBuffer packetBuffer = ByteBuffer.allocate(2).order(ByteOrder.LITTLE_ENDIAN);
        ByteBuffer lengthBuffer = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN);

        byte[] packetIdArray = packetBuffer.array();
        byte[] dataLengthArray = lengthBuffer.array();

        if (ReceiveSocket.read(packetIdArray) != -1 && ReceiveSocket.read(dataLengthArray) != -1) {
            int dataLength = lengthBuffer.getInt();
            byte[] data = new byte[dataLength];

            int dataRead = ReceiveSocket.read(data);

            if (dataRead != dataLength) {
                throw new Exception("Data length does not match. (Length: " + dataLength + " Interpreted: " + dataRead + ")");
            }

            return new Packet(data, packetBuffer.getShort());
        }

        return null;
    }

    public <T extends GeneratedMessageV3> @Nullable GeneratedMessageV3 SerializePacket(T packetType, Packet packet) throws Exception {
        return packetType.getParserForType().parseFrom(packet.data());
    }
}
