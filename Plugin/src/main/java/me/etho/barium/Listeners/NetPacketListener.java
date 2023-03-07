package me.etho.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListenerAbstract;
import com.github.retrooper.packetevents.event.PacketListenerPriority;
import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.event.PacketSendEvent;
import com.github.retrooper.packetevents.protocol.packettype.PacketType;
import com.github.retrooper.packetevents.wrapper.play.client.WrapperPlayClientPlayerFlying;
import me.etho.barium.Backend.Api.BariumApi;
import me.etho.barium.Backend.Packets.Play.PlayerMovement;
import me.etho.barium.Backend.Packets.Service.NodeUnregister;
import me.etho.barium.Backend.Utils.ApiUtils;
import me.etho.barium.Barium;

import java.io.IOException;

public class NetPacketListener extends PacketListenerAbstract {

    public NetPacketListener(PacketListenerPriority priority) {
        super(priority);
    }

    @Override
    public void onPacketReceive(PacketReceiveEvent event) {
        if (event.getPacketType() == PacketType.Play.Client.PLAYER_POSITION || event.getPacketType() == PacketType.Play.Client.PLAYER_POSITION_AND_ROTATION) {
            PacketReceiveEvent copy = event.clone();
            final WrapperPlayClientPlayerFlying wrappedFlying = new WrapperPlayClientPlayerFlying(copy);

            PlayerMovement playerMovement = new PlayerMovement();
            playerMovement.server_key = Barium.getApi().getServerKey();
            playerMovement.username = event.getUser().getName();
            playerMovement.x = wrappedFlying.getLocation().getX();
            playerMovement.y = wrappedFlying.getLocation().getY();
            playerMovement.z = wrappedFlying.getLocation().getZ();
            playerMovement.ground = wrappedFlying.isOnGround();

            try {
                Barium.getInstance().getLogger().info("Sending packet: X: " + wrappedFlying.getLocation().getX());
                ApiUtils.SendPacket(12, playerMovement, BariumApi.getInstance().getConnection());
            } catch (IOException ex) {
                Barium.getInstance().getLogger().warning("Barium API Exception: " + ex);
            }
            copy.cleanUp();
        }
    }

    @Override
    public void onPacketSend(PacketSendEvent event) {

    }
}
