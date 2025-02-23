package tech.strateim.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListener;
import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.protocol.ConnectionState;
import com.github.retrooper.packetevents.protocol.player.ClientVersion;
import com.github.retrooper.packetevents.protocol.player.User;
import com.google.protobuf.Message;
import tech.strateim.barium.Listeners.Handlers.MovementHandler;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Remote;

import java.net.SocketException;
import java.util.logging.Logger;

public class NetworkListener implements PacketListener {
    private final Remote Remote;
    private final Logger Log;

    public NetworkListener(Remote remote, Logger log) {
        Remote = remote;
        Log = log;
    }

    @Override
    public void onPacketReceive(PacketReceiveEvent event) {
        if (Remote.GetStateHandler() == null) {
            return;
        }

        if (Remote.GetStateHandler().State != Status.Ready || event.getConnectionState() != ConnectionState.PLAY) {
            return;
        }

        User user = event.getUser();

        ClientVersion clientVersion = user.getClientVersion();
        int id = event.getPacketId();

        InvokePacketMatch(user, clientVersion, id, event);
    }

    public void InvokePacketMatch(User user, ClientVersion clientVersion, int id, PacketReceiveEvent event) {
        String userUuid = user.getUUID().toString();

        if (MovementHandler.IsPosition(id, clientVersion)) {
            MovementHandler.HandlePosition(userUuid, event, this);
        }
    }

    public void HandlePacket(Message packet, int packetId) {
        if (packet == null) {
            return;
        }

        try {
            Remote.GetPacketHandler().SendPacket(packet, packetId, Remote.GetStateHandler().State);
        } catch (Exception ex){
            if (ex instanceof SocketException) {
                Remote.GetStateHandler().State = Status.Crash;
                Log.severe("Socket connection to governor lost!");
                Remote.Restart();
            } else {
                Log.warning("Exception occurred when handling a packet (" + ex + ")");
            }
        }
    }
}
