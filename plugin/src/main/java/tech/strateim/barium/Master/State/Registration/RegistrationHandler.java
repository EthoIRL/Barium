package tech.strateim.barium.Master.State.Registration;

import com.github.retrooper.packetevents.PacketEventsAPI;
import com.github.retrooper.packetevents.manager.server.ServerManager;
import init.Os;
import init.Protocol;
import init.Register;
import init.RegistrationResponse;
import org.bukkit.Server;
import org.bukkit.plugin.PluginManager;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.StateHandler;
import tech.strateim.barium.Master.Utilities.PacketEventsConversion;

import java.util.logging.Logger;

public class RegistrationHandler {
    private final StateHandler StateHandler;
    private final PacketHandler PacketHandler;
    private final Logger Log;

    public RegistrationHandler(PacketHandler packetHandler, StateHandler stateHandler, Logger log) {
        PacketHandler = packetHandler;
        StateHandler = stateHandler;
        Log = log;
    }

    public void InitRegister(Server localServer, PacketEventsAPI<?> packetEvents) {
        ServerManager serverManager = packetEvents.getServerManager();
        Os system = PacketEventsConversion.SystemConversion(serverManager.getOS());
        PluginManager pluginManager = localServer.getPluginManager();

        Protocol.Builder protocolBuilder = Protocol.newBuilder()
                .setGeyser(pluginManager.isPluginEnabled("Geyser-Spigot"))
                .setViaBackwards(pluginManager.isPluginEnabled("ViaBackwards"))
                .setViaRewind(pluginManager.isPluginEnabled("ViaRewind"))
                .setViaVersion(pluginManager.isPluginEnabled("ViaVersion"));

        Register register = Register.newBuilder()
                .setPluginVersion(0)
                .setServerVersion(serverManager.getVersion().getProtocolVersion())
                .setOs(system)
                .setProtocol(protocolBuilder)
                .build();

        PacketHandler.SendPacket(register, 0);
    }

    public void HandleResponse(Packet packet) {
        try {
            RegistrationResponse response = (RegistrationResponse) PacketHandler.SerializePacket(RegistrationResponse.getDefaultInstance(), packet);

            if (response == null) {
                return;
            }

            if (response.getSucceeded()) {
                Log.info(response.toString());

                StateHandler.State = Status.Ready;

                return;
            }

            Log.severe("Failed to authenticate, registration failed");
            Log.info(response.toString());

            StateHandler.State = Status.Shutdown;
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
