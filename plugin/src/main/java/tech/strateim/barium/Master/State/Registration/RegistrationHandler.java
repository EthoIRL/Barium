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
    private final PacketHandler PacketHandler;
    private final Logger Log;

    public RegistrationHandler(int id, PacketHandler packetHandler, StateHandler stateHandler, Logger log) {
        super(id);
        PacketHandler = packetHandler;
        StateHandler = stateHandler;
        Log = log;
    }

    public void HandleResponse(Packet packet) {
        try {
            RegistrationResponse response = (RegistrationResponse) PacketHandler.SerializePacket(RegistrationResponse.getDefaultInstance(), packet);

            if (response == null) {
                return;
            }

            if (response.getSucceeded()) {
                Log.info(response.toString());

                StateHandler.State = Status.Registered;
                StateHandler.Key = response.getUuidKey();

                return;
            }

            Log.severe("Failed to authenticate, registration failed");
            Log.info(response.toString());

            StateHandler.State = Status.Crash;
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
