package tech.strateim.barium.Master.State.Registration;

import init.RegistrationResponse;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.AbstractState;
import tech.strateim.barium.Master.State.StateHandler;

import java.util.logging.Logger;

public class RegistrationHandler extends AbstractState {
    private final StateHandler StateHandler;

    public RegistrationHandler(int id, PacketHandler packetHandler, Logger log, StateHandler stateHandler) {
        super(id, packetHandler, log);
        StateHandler = stateHandler;
    }

    public void HandleResponse(Packet packet) throws Exception {
        RegistrationResponse response = (RegistrationResponse) packetHandler.SerializePacket(RegistrationResponse.getDefaultInstance(), packet);

        if (response == null) {
            return;
        }

        if (response.getSucceeded()) {
            log.info(response.toString());

            StateHandler.State = Status.Registered;
            StateHandler.Key = response.getUuidKey();

            return;
        }

        log.severe("Failed to authenticate, registration failed");
        log.info(response.toString());

        StateHandler.State = Status.Crash;
    }
}
