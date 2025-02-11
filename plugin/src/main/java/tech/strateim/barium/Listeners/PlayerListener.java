package tech.strateim.barium.Listeners;

import com.github.retrooper.packetevents.event.PacketListener;
import com.github.retrooper.packetevents.event.UserDisconnectEvent;
import com.github.retrooper.packetevents.event.UserLoginEvent;
import tech.strateim.barium.Master.Remote;


public class PlayerListener implements PacketListener {
    private Remote Remote;

    public PlayerListener(Remote remote) {
        Remote = remote;
    }

    @Override
    public void onUserLogin(UserLoginEvent event) {
    }

    @Override
    public void onUserDisconnect(UserDisconnectEvent event) {
    }
}
