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

    public Socket connection = null;

    public BariumApi() {
        instance = this;
    }

    public void InitBackend(String serverVersion, String serverOS, boolean ViaVersion) {
        try {
            NodeRegister register = new NodeRegister();
            register.server_ip = "****";
            register.server_version = serverVersion;
            register.server_region = NodeRegister.Region.NA;
            register.server_os = serverOS;
            register.via_version = ViaVersion;
            Packet<NodeKey> nodekey = ApiUtils.SendAndReceivePacket(0, register, NodeKey.class, getConnection());
            Barium.getInstance().getLogger().info("KEY: " + nodekey.getPacket().key + " ID: " + nodekey.getId());

            ServerKey = nodekey.getPacket().key;
        } catch (Exception ex) {
            Barium.getInstance().getLogger().warning("Barium API Exception: " + ex);
        }
    }

    public void Connect() throws IOException {
        int serverPort = 8080;
        InetAddress host = InetAddress.getByName("127.0.0.1");
        Socket socket = new Socket(host, serverPort);
        socket.setSoTimeout(30000);

        connection = socket;
    }

    public Socket getConnection() throws IOException {
        if (connection == null) {
            Connect();
        }

        return connection;
    }
}
