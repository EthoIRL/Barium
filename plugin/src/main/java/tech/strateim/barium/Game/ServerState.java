package tech.strateim.barium.Game;

import com.github.retrooper.packetevents.PacketEventsAPI;
import com.github.retrooper.packetevents.protocol.player.User;
import org.bukkit.Server;
import org.bukkit.entity.Player;
import tech.strateim.barium.Game.Clients.Client;

import java.util.Collection;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.logging.Logger;

public class ServerState {
    private static Logger Log;
    private final ConcurrentHashMap<UUID, Client> ClientMap = new ConcurrentHashMap<>();

    public ServerState(Logger log, Server server, PacketEventsAPI<?> peAPI) {
        Log = log;

        server.getOnlinePlayers().forEach(player -> {
            var peUser = peAPI.getPlayerManager().getUser(player);

            ClientMap.put(player.getUniqueId(), new Client(player, peUser));
        });
    }

    public Client getClient(final Player player) {
        return ClientMap.get(player.getUniqueId());
    }

    public Client getClient(final String uuid) {
        return ClientMap.get(UUID.fromString(uuid));
    }

    public void removeClient(final Player player) {
        ClientMap.remove(player.getUniqueId());
    }

    public void removeClient(final UUID uuid) {
        ClientMap.remove(uuid);
    }

    public void addClient(final UUID uuid, Player player, User peUser) {
        ClientMap.put(uuid, new Client(player, peUser));
    }

    public Collection<Client> getAllClients() {
        return ClientMap.values();
    }
}
