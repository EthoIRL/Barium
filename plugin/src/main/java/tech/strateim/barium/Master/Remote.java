package tech.strateim.barium.Master;

import com.github.retrooper.packetevents.PacketEventsAPI;
import com.github.retrooper.packetevents.manager.server.ServerManager;
import generic.DisconnectReason;
import generic.Os;
import generic.Protocol;
import org.bukkit.Server;
import org.bukkit.plugin.PluginManager;
import server.DisconnectServer;
import server.ServerRegistration;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.StateHandler;
import tech.strateim.barium.Master.Utilities.PacketEventsConversion;

import javax.annotation.Nullable;
import java.io.InputStream;
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
    private InputStream SocketReceive;
    public final ExecutorService ExecutorService = Executors.newFixedThreadPool(8);
    private PacketHandler PacketHandler;
    private StateHandler StateHandler;

    public Remote(Logger log) {
        Log = log;
    }

    public @Nullable PacketHandler Start(String governorAddress, int governorPort, Server localServer, PacketEventsAPI<?> packetEvents) {
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
            Log.log(Level.SEVERE, "Failed to get sender socket stream to governor server");

            return null;
        }

        try {
            SocketReceive = Socket.getInputStream();
        } catch (Exception ex) {
            Log.log(Level.SEVERE, "Failed to get receiver socket stream to governor server");

            return null;
        }

        PacketHandler = new PacketHandler(Log, SocketOutput, SocketReceive);
        StateHandler = new StateHandler(PacketHandler, SocketOutput, SocketReceive, Log, this);

        ExecutorService.execute(StateHandler::StartReceiver);
        ExecutorService.execute(() -> InitRegistration(localServer, packetEvents));

        return PacketHandler;
    }

    public void Shutdown() {
        ExecutorService.shutdown();
    }

    public void Disconnect(DisconnectReason reason) {
        DisconnectServer.Builder disconnectBuilder = DisconnectServer.newBuilder()
                .setReason(reason);

        if (StateHandler.Key != null) {
            disconnectBuilder.setUuidKey(StateHandler.Key);
        }

        PacketHandler.SendPacket(disconnectBuilder.build(), 2);
    }

    private void InitRegistration(Server localServer, PacketEventsAPI<?> packetEvents) {
        ServerManager serverManager = packetEvents.getServerManager();
        Os system = PacketEventsConversion.SystemConversion(serverManager.getOS());
        PluginManager pluginManager = localServer.getPluginManager();

        Protocol.Builder protocolBuilder = Protocol.newBuilder()
                .setGeyser(pluginManager.isPluginEnabled("Geyser-Spigot"))
                .setViaBackwards(pluginManager.isPluginEnabled("ViaBackwards"))
                .setViaRewind(pluginManager.isPluginEnabled("ViaRewind"))
                .setViaVersion(pluginManager.isPluginEnabled("ViaVersion"));

        ServerRegistration.Register register = ServerRegistration.Register.newBuilder()
                .setPluginVersion(0)
                .setServerVersion(serverManager.getVersion().getProtocolVersion())
                .setOs(system)
                .setProtocol(protocolBuilder)
                .build();

        PacketHandler.SendPacket(register, 0);
    }
}