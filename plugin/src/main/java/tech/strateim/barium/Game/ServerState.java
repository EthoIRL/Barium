package tech.strateim.barium.Game;

import com.github.retrooper.packetevents.protocol.player.User;

import java.util.HashMap;
import java.util.UUID;
import java.util.logging.Logger;

public class ServerState {
    private static Logger Log;
    private final HashMap<UUID, User> Users = new HashMap<>();

    public ServerState(Logger log) {
        Log = log;
    }

    public HashMap<UUID, User> getUsers() {
        return Users;
    }
}
