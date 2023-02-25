package me.etho.barium;

import com.github.retrooper.packetevents.PacketEvents;
import com.github.retrooper.packetevents.event.PacketListenerPriority;
import io.github.retrooper.packetevents.factory.spigot.SpigotPacketEventsBuilder;
import lombok.Getter;
import me.etho.barium.Listeners.NetPacketListener;
import me.etho.barium.Listeners.PlayerPacketListener;
import org.bukkit.plugin.java.JavaPlugin;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public final class Barium extends JavaPlugin {

    @Getter
    private static Barium instance;

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

        if (PacketEvents.getAPI().isInitialized())
            getLogger().info("PacketEvents initialized & running " + PacketEvents.getAPI().getVersion().toString() + "!");
    }

    @Override
    public void onDisable() {
        PacketEvents.getAPI().terminate();
    }
}
