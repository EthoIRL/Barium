package tech.strateim.barium.Master.State.Ready;

import network.Px_PlayerJoin;
import tech.strateim.barium.Game.Clients.Client;
import tech.strateim.barium.Game.ServerState;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.AbstractState;
import tech.strateim.barium.Master.State.StateHandler;

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

        for (Client client: serverState.getAllClients()) {
            log.warning("Registering: " + client.getPlayer().getName());

            Px_PlayerJoin playerJoin = Px_PlayerJoin.newBuilder()
                    .setUuid(client.getUser().getUUID().toString())
                    .setName(client.getUser().getName())
                    .build();

            packetHandler.SendPacketRetry(playerJoin, 1, stateHandler.State);
        }
    }
}
