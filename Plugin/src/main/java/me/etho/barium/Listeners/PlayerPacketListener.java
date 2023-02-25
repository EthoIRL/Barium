package me.etho.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListenerAbstract;
import com.github.retrooper.packetevents.event.PacketListenerPriority;
import com.github.retrooper.packetevents.event.UserDisconnectEvent;
import com.github.retrooper.packetevents.event.UserLoginEvent;
import me.etho.barium.Barium;

public class PlayerPacketListener extends PacketListenerAbstract {

    public PlayerPacketListener(PacketListenerPriority priority) {
        super(priority);
    }

    @Override
    public void onUserLogin(UserLoginEvent event) {
        Barium.getInstance().getLogger().info("user logged in " + event.getUser().getName());
    }

    @Override
    public void onUserDisconnect(UserDisconnectEvent event) {
        Barium.getInstance().getLogger().info("user logged out " + event.getUser().getName());
    }
}
