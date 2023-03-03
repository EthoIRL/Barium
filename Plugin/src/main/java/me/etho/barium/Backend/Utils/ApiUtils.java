package me.etho.barium.Backend.Utils;

import com.google.gson.Gson;
import me.etho.barium.Backend.Packets.Packet;
import me.etho.barium.Barium;

import java.io.*;
import java.net.Socket;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;

public class ApiUtils {

    public static Gson gson = new Gson();

    public static <T> void SendPacket(int id, T packet, Socket socket) throws IOException {
        String jsonPacket = gson.toJson(packet);

        BufferedOutputStream writer = new BufferedOutputStream(socket.getOutputStream());
        writer.write(id);
        writer.write(jsonPacket.getBytes(StandardCharsets.UTF_8));
        writer.flush();
    }

//    public static <S> Packet SendPacketAndReceive(int id, S packet, Socket socket) throws IOException {
//        SendPacket(id, packet, socket);
//
//        DataInputStream reader = new DataInputStream(socket.getInputStream());
//        byte packetId = reader.readByte();
//        byte[] buffer = new byte[1024];
//
//        int lengthRead = reader.read(buffer);
//        buffer = Arrays.copyOf(buffer, lengthRead);
//
//        return new Packet(packetId, buffer);
//    }

    public static <S, R> Packet<R> SendAndReceivePacket(int id, S sendPacket, Class<R> receivePacket, Socket socket) throws IOException {
        SendPacket(id, sendPacket, socket);

        DataInputStream reader = new DataInputStream(socket.getInputStream());
        byte packetId = reader.readByte();
        byte[] buffer = new byte[1024];

        int lengthRead = reader.read(buffer);
        buffer = Arrays.copyOf(buffer, lengthRead);

        R packet = gson.fromJson(new String(buffer, StandardCharsets.UTF_8), receivePacket);

        return new Packet<>(packetId, packet);
    }
}
 