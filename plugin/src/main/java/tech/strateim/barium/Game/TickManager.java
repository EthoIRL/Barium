package tech.strateim.barium.Game;

import network.Px_ServerTick;
import org.bukkit.Bukkit;
import org.bukkit.scheduler.BukkitTask;
import tech.strateim.barium.Barium;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Remote;

public class TickManager implements Runnable {
    private final Barium barium;
    private final Remote remote;
    private BukkitTask task;
    private final Px_ServerTick ServerTick = Px_ServerTick.newBuilder().build();

    public TickManager(Barium barium, Remote remote) {
        this.barium = barium;
        this.remote = remote;
    }
    public void Start() {
        task = Bukkit.getScheduler().runTaskTimerAsynchronously(barium, this, 0L, 1L);
    }

    public void Stop() {
        if (task == null) {
            return;
        }
        task.cancel();
        task = null;
    }

    @Override
    public void run() {
        if (remote.GetStateHandler().State == Status.Ready) {
            remote.ExecutorService.execute(() -> {
                remote.GetPacketHandler().SendPacketRetry(ServerTick, 7, remote.GetStateHandler().State);
            });
        }
    }
}
