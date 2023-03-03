package me.etho.barium.Backend.Api;

import lombok.Getter;
import me.etho.barium.Backend.Packets.Packet;
import me.etho.barium.Backend.Packets.Service.NodeKey;
import me.etho.barium.Backend.Packets.Service.NodeRegister;
import me.etho.barium.Backend.Utils.ApiUtils;
import me.etho.barium.Barium;

import java.io.IOException;
import java.net.InetAddress;
import java.net.Socket;
import java.net.UnknownHostException;
import java.nio.charset.StandardCharsets;

public class BariumApi {

    @Getter
    public static BariumApi instance;

    @Getter
    public String ServerKey = null;

    public BariumApi() {
        instance = this;
    }

    public void InitBackend() {
        try {
            int serverPort = 8080;
            InetAddress host = InetAddress.getByName("127.0.0.1");
            Socket remoteSocket = new Socket(host, serverPort);

            NodeRegister register = new NodeRegister();
            register.server_ip = "127.0.0.1";
            register.server_version = "1.8.9";
            register.server_region = NodeRegister.Region.NA;
            register.via_version = true;
            register.bungee_cord = false;
            register.cracked = false;
            Packet<NodeKey> nodekey = ApiUtils.SendAndReceivePacket(0, register, NodeKey.class, remoteSocket);
            Barium.getInstance().getLogger().info("KEY: " + nodekey.getPacket().key + " ID: " + nodekey.getId());

            ServerKey = nodekey.getPacket().key;
//            Packet packet = ApiUtils.SendPacketAndReceive(0, register, remoteSocket);
//            Barium.getInstance().getLogger().info(new String(packet.getData(), StandardCharsets.UTF_8));
        } catch (Exception ex) {
            Barium.getInstance().getLogger().warning("Barium API Exception: " + ex);
        }
    }

    public Socket Connect() throws IOException {
        int serverPort = 8080;
        InetAddress host = InetAddress.getByName("127.0.0.1");
        Socket socket = new Socket(host, serverPort);
        socket.setSoTimeout(30000);

        return socket;
    }
}
