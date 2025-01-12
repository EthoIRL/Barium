package tech.strateim.barium;

import com.github.retrooper.packetevents.PacketEvents;
import com.github.retrooper.packetevents.PacketEventsAPI;
import com.github.retrooper.packetevents.settings.PacketEventsSettings;
import com.github.retrooper.packetevents.util.TimeStampMode;
import generic.DisconnectReason;
import io.github.retrooper.packetevents.factory.spigot.SpigotPacketEventsBuilder;
import org.bukkit.Server;
import org.bukkit.plugin.java.JavaPlugin;
import tech.strateim.barium.Master.Remote;
import tech.strateim.barium.Master.Packet.PacketHandler;

import java.util.logging.Logger;

public final class Barium extends JavaPlugin {
    public Logger Log;
    public PacketEventsAPI<?> PeApi;

    public Server Server;
    public Remote Remote;
    public PacketHandler PacketHandler;

    @Override
    public void onLoad() {
        Log = getLogger();
        Server = getServer();

        Remote = new Remote(this, Log);

        PacketEvents.setAPI(SpigotPacketEventsBuilder.build(this));
        PacketEvents.getAPI().load();

        PeApi = PacketEvents.getAPI();

        Remote.ExecutorService.execute(() -> PacketHandler = Remote.Start("127.0.0.1", 3238, Server, PeApi));

        PacketEventsSettings settings = PeApi.getSettings();
        settings.checkForUpdates(false).debug(true).timeStampMode(TimeStampMode.NANO);
    }

    @Override
    public void onEnable() {
        PacketEvents.getAPI().init();
    }

    @Override
    public void onDisable() {
        Remote.Disconnect(DisconnectReason.Shutdown);
        Remote.Shutdown();

        PacketEvents.getAPI().terminate();
    }
}
