package tech.strateim.barium.Game.Clients;

import org.bukkit.entity.Player;

public class Client {
    private final Player player;
    private final com.github.retrooper.packetevents.protocol.player.User user;
    private final boolean alerts;
    private int violations = 0;

    public Client(Player player, com.github.retrooper.packetevents.protocol.player.User user) {
        this.player = player;
        this.user = user;

        alerts = player.isOp();
    }

    public Player getPlayer() {
        return player;
    }

    public com.github.retrooper.packetevents.protocol.player.User getUser() {
        return user;
    }

    public boolean isAlerts() {
        return alerts;
    }

    public int getViolations() {
        return violations;
    }

    public void addViolations(int vl) {
        violations += vl;
    }
}
