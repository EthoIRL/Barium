package me.etho.barium;

import com.github.retrooper.packetevents.PacketEvents;
import com.github.retrooper.packetevents.event.PacketListenerPriority;
import io.github.retrooper.packetevents.factory.spigot.SpigotPacketEventsBuilder;
import lombok.Getter;
import me.etho.barium.Backend.Api.BariumApi;
import me.etho.barium.Backend.Packets.Play.PlayerLeave;
import me.etho.barium.Backend.Packets.Service.NodeUnregister;
import me.etho.barium.Backend.Utils.ApiUtils;
import me.etho.barium.Listeners.NetPacketListener;
import me.etho.barium.Listeners.PlayerPacketListener;
import org.bukkit.plugin.java.JavaPlugin;

import java.io.IOException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public final class Barium extends JavaPlugin {

    @Getter
    private static Barium instance;

    @Getter
    private static BariumApi api;

    @Getter
    private static final ExecutorService PacketExecutor = Executors.newSingleThreadExecutor();

    @Override
    public void onLoad() {
        instance = this;
        PacketEvents.setAPI(SpigotPacketEventsBuilder.build(this));

        PacketEvents.getAPI().getSettings()
                .readOnlyListeners(true)
                .checkForUpdates(false)
                .bStats(true);

        PacketEvents.getAPI().load();
    }

    @Override
    public void onEnable() {
        PacketEvents.getAPI().getEventManager().registerListener(new NetPacketListener(PacketListenerPriority.HIGHEST));
        PacketEvents.getAPI().getEventManager().registerListener(new PlayerPacketListener(PacketListenerPriority.HIGHEST));

        PacketEvents.getAPI().init();

        if (PacketEvents.getAPI().isInitialized()) {
            getLogger().info("PacketEvents initialized & running " + PacketEvents.getAPI().getVersion().toString() + "!");
            api = new BariumApi();

            api.InitBackend();
        } else {
            getLogger().warning("PacketEvents failed to initialized " + PacketEvents.getAPI().getVersion().toString() + "!");
            getServer().getPluginManager().disablePlugin(this);
        }
    }

    @Override
    public void onDisable() {
        PacketEvents.getAPI().terminate();

        NodeUnregister nodeUnregister = new NodeUnregister();
        nodeUnregister.key = Barium.getApi().getServerKey();

        try {
            ApiUtils.SendPacket(1, nodeUnregister, BariumApi.getInstance().Connect());
        } catch (IOException ex) {
            Barium.getInstance().getLogger().warning("Barium API Exception: " + ex);
        }
    }
}
