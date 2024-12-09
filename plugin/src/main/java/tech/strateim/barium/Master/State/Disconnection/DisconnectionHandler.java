package tech.strateim.barium.Master.State.Disconnection;

import init.Disconnect;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.Remote;
import tech.strateim.barium.Master.State.AbstractState;
import tech.strateim.barium.Master.State.StateHandler;

import java.util.Objects;
import java.util.logging.Logger;

public class DisconnectionHandler extends AbstractState {

    private final Remote remote;
    private final StateHandler stateHandler;

    public DisconnectionHandler(int id, PacketHandler packetHandler, Logger log, Remote remote, StateHandler stateHandler) {
        super(id, packetHandler, log);
        this.remote = remote;
        this.stateHandler = stateHandler;
    }

    @Override
    public void HandleResponse(Packet packet) throws Exception {
        Disconnect response = (Disconnect) packetHandler.SerializePacket(Disconnect.getDefaultInstance(), packet);

        if (response == null) {
            return;
        }

        if (stateHandler.State == Status.Initialization) {
            assert (response.hasUuidKey());
        }

        if (!Objects.equals(stateHandler.Key, response.getUuidKey())) {
            log.severe("Remote shutdown attempted however invalid Uuid Key presented!");
            return;
        }

        log.info("Remote shutdown initiated (Reason: " + response.getReason() + ")");

        remote.Shutdown();
    }
}
