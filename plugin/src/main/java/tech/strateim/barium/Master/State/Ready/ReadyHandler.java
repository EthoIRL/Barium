package tech.strateim.barium.Master.State.Ready;

import server.ProxyMessage;
import server.ServerRegistration;
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

        while(true) {
            ProxyMessage message = ProxyMessage.newBuilder()
                    .build();

            packetHandler.SendPacket(message, 10);

            Thread.sleep(500);
        }

        // TODO: Begin sending packets to anticheat
        // TODO: Register all connected players with anticheat
    }
}
