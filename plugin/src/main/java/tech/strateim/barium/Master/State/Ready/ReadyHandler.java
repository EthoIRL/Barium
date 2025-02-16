package tech.strateim.barium.Master.State.Ready;

import com.github.retrooper.packetevents.protocol.player.User;
import network.Px_PlayerJoin;
import tech.strateim.barium.Game.ServerState;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.AbstractState;
import tech.strateim.barium.Master.State.StateHandler;

import java.util.HashMap;
import java.util.UUID;
import java.util.logging.Logger;

public class ReadyHandler extends AbstractState {

    private final StateHandler stateHandler;
    private final ServerState serverState;

    public ReadyHandler(int id, PacketHandler packetHandler, Logger log, ServerState serverState, StateHandler stateHandler) {
        super(id, packetHandler, log);
        this.stateHandler = stateHandler;
        this.serverState = serverState;
    }

    @Override
    public void HandleResponse(Packet packet) throws Exception {
        log.warning("READY TO SEND");
        stateHandler.State = Status.Ready;

        HashMap<UUID, User> users = serverState.getUsers();

        for (UUID key: users.keySet()) {
            User user = users.get(key);

            log.warning("Registering: " + user.getName());

            Px_PlayerJoin playerJoin = Px_PlayerJoin.newBuilder()
                    .setUuid(key.toString())
                    .setName(user.getName())
                    .build();

            packetHandler.SendPacketRetry(playerJoin, 0, stateHandler.State);
        }
    }
}
