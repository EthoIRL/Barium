package tech.strateim.barium.Master.State.Ready;

import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.AbstractState;
import tech.strateim.barium.Master.State.StateHandler;

import java.util.logging.Logger;

public class ReadyHandler extends AbstractState {

    private final StateHandler stateHandler;

    public ReadyHandler(int id, PacketHandler packetHandler, Logger log, StateHandler stateHandler) {
        super(id, packetHandler, log);
        this.stateHandler = stateHandler;
    }

    @Override
    public void HandleResponse(Packet packet) throws Exception {
        stateHandler.State = Status.Ready;

        log.warning("READY TO SEND");
        // TODO: Begin sending packets to anticheat
        // TODO: Register all connected players with anticheat
    }
}
