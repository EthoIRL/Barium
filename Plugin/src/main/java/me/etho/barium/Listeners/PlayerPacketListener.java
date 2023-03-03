package me.etho.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListenerAbstract;
import com.github.retrooper.packetevents.event.PacketListenerPriority;
import com.github.retrooper.packetevents.event.UserDisconnectEvent;
import com.github.retrooper.packetevents.event.UserLoginEvent;
import me.etho.barium.Backend.Api.BariumApi;
import me.etho.barium.Backend.Packets.Packet;
import me.etho.barium.Backend.Packets.Play.PlayerJoin;
import me.etho.barium.Backend.Packets.Service.NodeKey;
import me.etho.barium.Backend.Utils.ApiUtils;
import me.etho.barium.Barium;

import java.io.IOException;

public class PlayerPacketListener extends PacketListenerAbstract {

    public PlayerPacketListener(PacketListenerPriority priority) {
        super(priority);
    }

    @Override
    public void onUserLogin(UserLoginEvent event) {
        Barium.getInstance().getLogger().info("user logged in " + event.getUser().getName());

        if (event.getUser().getName() != null) {
            PlayerJoin playerJoin = new PlayerJoin();
            playerJoin.server_key = Barium.getApi().getServerKey();
            playerJoin.username = event.getUser().getName();
            playerJoin.uuid = event.getUser().getUUID().toString();
            playerJoin.version = event.getUser().getClientVersion().toString();

            try {
                ApiUtils.SendPacket(10, playerJoin, BariumApi.getInstance().Connect());
            } catch (IOException ex) {
                Barium.getInstance().getLogger().warning("Barium API Exception: " + ex);
            }
        }

    }

    @Override
    public void onUserDisconnect(UserDisconnectEvent event) {
        Barium.getInstance().getLogger().info("user logged out " + event.getUser().getName());
    }
}
