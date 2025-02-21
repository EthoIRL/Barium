package tech.strateim.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListener;
import com.github.retrooper.packetevents.event.UserDisconnectEvent;
import com.github.retrooper.packetevents.event.UserLoginEvent;
import com.github.retrooper.packetevents.protocol.player.User;
import network.Px_PlayerJoin;
import network.Px_PlayerLeave;
import tech.strateim.barium.Game.ServerState;
import tech.strateim.barium.Master.Remote;

import java.util.UUID;

public class PlayerListener implements PacketListener {
    private Remote Remote;
    private ServerState ServerState;

    public PlayerListener(Remote remote, ServerState serverState) {
        Remote = remote;
        ServerState = serverState;
    }

    @Override
    public void onUserLogin(UserLoginEvent event) {
        User user = event.getUser();
        UUID userUuid = user.getUUID();

        if (userUuid == null) {
            return;
        }

        Px_PlayerJoin playerJoin = Px_PlayerJoin.newBuilder()
                .setUuid(userUuid.toString())
                .setName(user.getName())
                .build();

        ServerState.getUsers().put(userUuid, user);

        Remote.GetPacketHandler().SendPacketRetry(playerJoin, 1, Remote.GetStateHandler().State);
    }

    @Override
    public void onUserDisconnect(UserDisconnectEvent event) {
        User user = event.getUser();
        UUID userUuid = user.getUUID();

        if (userUuid == null) {
            return;
        }

        Px_PlayerLeave playerLeave = Px_PlayerLeave.newBuilder()
                .setUuid(userUuid.toString())
                .build();

        ServerState.getUsers().remove(userUuid);

        Remote.GetPacketHandler().SendPacketRetry(playerLeave, 2, Remote.GetStateHandler().State);
    }
}
