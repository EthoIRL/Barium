package tech.strateim.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListener;
import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.protocol.ConnectionState;
import com.github.retrooper.packetevents.protocol.player.ClientVersion;
import com.github.retrooper.packetevents.protocol.player.User;
import tech.strateim.barium.Listeners.Handlers.GenericPacket;
import tech.strateim.barium.Listeners.Handlers.MovementHandler;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.Remote;
import tech.strateim.barium.Master.State.StateHandler;

import javax.annotation.Nullable;
import java.net.SocketException;
import java.util.logging.Logger;

public class NetworkListener implements PacketListener {

    private final Remote Remote;
    private final Logger Log;
    private final StateHandler stateHandler;
    private final PacketHandler packetHandler;

    public NetworkListener(Remote remote, Logger log) {
        Remote = remote;
        Log = log;

        stateHandler = remote.GetStateHandler();
        packetHandler = remote.GetPacketHandler();
    }

    @Override
    public void onPacketReceive(PacketReceiveEvent event) {
        if (stateHandler.State != Status.Ready || event.getConnectionState() != ConnectionState.PLAY) {
            return;
        }

        User user = event.getUser();

        ClientVersion clientVersion = user.getClientVersion();
        int id = event.getPacketId();

        HandlePacket(InvokePacketMatch(user, clientVersion, id, event));
    }

    @Nullable
    public GenericPacket InvokePacketMatch(User user, ClientVersion clientVersion, int id, PacketReceiveEvent event) {
        String userUuid = user.getUUID().toString();

        if (MovementHandler.IsPosition(id, clientVersion)) {
            return MovementHandler.HandlePosition(userUuid, event);
        }

        return null;
    }

    public void HandlePacket(@Nullable GenericPacket packet) {
        if (packet == null) {
            return;
        }

        try {
            packetHandler.SendPacket(packet.getPacketData(), packet.getPacketId(), stateHandler.State);
        } catch (Exception ex){
            if (ex instanceof SocketException) {
                stateHandler.State = Status.Crash;
                Log.severe("Socket connection to governor lost!");
                Remote.Restart();
            } else {
                Log.warning("Exception occurred when handling a packet (" + ex + ")");
            }
        }
    }
}
