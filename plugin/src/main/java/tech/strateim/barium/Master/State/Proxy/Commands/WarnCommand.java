package tech.strateim.barium.Master.State.Proxy.Commands;

import com.github.retrooper.packetevents.protocol.player.User;
import command.Px_PlayerWarn;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.event.ClickEvent;
import net.kyori.adventure.text.event.HoverEvent;
import org.bukkit.ChatColor;
import tech.strateim.barium.Game.Clients.Client;
import tech.strateim.barium.Game.ServerState;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.AbstractState;

import java.util.logging.Logger;

public class WarnCommand extends AbstractState {
    private final ServerState serverState;

    public WarnCommand(int id, PacketHandler packetHandler, Logger log, ServerState serverState) {
        super(id, packetHandler, log);
        this.serverState = serverState;
    }

    @Override
    public void HandleResponse(Packet packet) throws Exception {
        Px_PlayerWarn playerWarn = (Px_PlayerWarn) packetHandler.SerializePacket(Px_PlayerWarn.getDefaultInstance(), packet);

        if (playerWarn == null) {
            return;
        }

        Client client = serverState.getClient(playerWarn.getUuid());
        client.addViolations(playerWarn.getCheckWeight());

        String alertMessage = ChatColor.translateAlternateColorCodes('&', formatAlert(playerWarn, client));
        String alertSubMessage = ChatColor.translateAlternateColorCodes('&', formatSubAlert(playerWarn, client));

        Component alert = Component.text(alertMessage)
                .hoverEvent(HoverEvent.showText(Component.text(alertSubMessage)))
                .clickEvent(ClickEvent.runCommand("/tp " + client.getPlayer().getName()));

        serverState.getAllClients().forEach(Client -> {
            if (Client.isAlerts()) {
                Client.getUser().sendMessage(alert);
            }
        });
    }

    private static String formatAlert(Px_PlayerWarn playerWarn, Client client) {
        User user = client.getUser();
        String alertMessage = "&8[&l&dBARIUM&8] &7%s &8failed &o&b%s" + (playerWarn.getCheckExperimental() ? "§aΔ" : "") + " &8[&7+%d &l&c/ &6%d&8]";

        return String.format(alertMessage, user.getName(), playerWarn.getCheckName(), playerWarn.getCheckWeight(), client.getViolations());
    }

    private static String formatSubAlert(Px_PlayerWarn playerWarn, Client client) {
        User user = client.getUser();

        String subMessage =
                """
                 &7Category: &b%s
                 &7Experimental: &6%b
                 &7Weight: &b%d
                 &7Client: &b%s
                 &6Click to teleport.""";

        return String.format(subMessage, playerWarn.getCheckType(), playerWarn.getCheckExperimental(), playerWarn.getCheckWeight(), user.getClientVersion().getReleaseName());
    }

}
