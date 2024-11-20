package tech.strateim.barium.Master;

import init.Os;
import init.Protocol;
import init.Register;
import org.bukkit.Server;

import java.io.OutputStream;
import java.net.Socket;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.logging.Level;
import java.util.logging.Logger;

public class Remote {
    private static Logger Log;
    private Socket Socket;
    private OutputStream SocketOutput;
    public final ExecutorService ExecutorService = Executors.newFixedThreadPool(8);
    public Status State = Status.Init;

    public Remote(Logger log) {
        Log = log;
    }

    public void Start(String governorAddress, int governorPort, Server localServer) {
        while (true) {
            try {
                Socket = new Socket(governorAddress, governorPort);
                Log.info("Connected to governor server");

                break;
            } catch (Exception ex) {
                Log.severe("Failed to connect to governor server");

                try {
                    Thread.sleep(5000);
                } catch (Exception ignore) {}
            }
        }

        try {
            SocketOutput = Socket.getOutputStream();
        } catch (Exception ex) {
            Log.log(Level.SEVERE, "Failed to get socket stream to governor server");
            State = Status.Crash;

            return;
        }

        ExecutorService.execute(() -> InitRegister(localServer));
    }

    private void InitRegister(Server localServer) {
        Protocol.Builder protocolBuilder = Protocol.newBuilder()
                .setGeyser(false)
                .setViaBackwards(false)
                .setViaRewind(false)
                .setViaVersion(false);

        Register register = Register.newBuilder()
                .setPluginVersion(0)
                .setServerVersion(134)
                .setOs(Os.Windows)
                .setProtocol(protocolBuilder)
                .setIpAddress("lol idk")
                .build();

        Packet.SendPacket(register, 0, SocketOutput, Log);
    }
}