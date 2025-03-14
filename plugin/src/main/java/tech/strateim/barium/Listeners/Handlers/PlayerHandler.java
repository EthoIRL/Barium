package tech.strateim.barium.Listeners.Handlers;

import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.protocol.packettype.PacketType;
import com.github.retrooper.packetevents.protocol.player.ClientVersion;
import com.github.retrooper.packetevents.wrapper.play.client.WrapperPlayClientPlayerAbilities;
import player.Px_PlayerClientAbilities;
import tech.strateim.barium.Game.Clients.Client;
import tech.strateim.barium.Listeners.NetworkListener;

public class PlayerHandler {
    public static void HandleClientAbilities(String userUuid, PacketReceiveEvent event, NetworkListener networkListener, Client client) {
        WrapperPlayClientPlayerAbilities abilities = new WrapperPlayClientPlayerAbilities(event);

        Px_PlayerClientAbilities playerAbilities = Px_PlayerClientAbilities.newBuilder()
                .setUuid(userUuid)
                .setFlying(abilities.isFlying())
                .setServerAllowed(client.getPlayer().getAllowFlight())
                .build();

        networkListener.HandlePacket(playerAbilities, 5);
    }

    public static boolean IsClientAbilities(int id, ClientVersion clientVersion) {
        return id == PacketType.Play.Client.PLAYER_ABILITIES.getId(clientVersion);
    }
}
