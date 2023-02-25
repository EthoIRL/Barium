package me.etho.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListenerAbstract;
import com.github.retrooper.packetevents.event.PacketListenerPriority;
import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.event.PacketSendEvent;

public class NetPacketListener extends PacketListenerAbstract {

    public NetPacketListener(PacketListenerPriority priority) {
        super(priority);
    }

    @Override
    public void onPacketReceive(PacketReceiveEvent event) {

    }

    @Override
    public void onPacketSend(PacketSendEvent event) {

    }
}
