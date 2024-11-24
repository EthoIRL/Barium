package tech.strateim.barium.Master.State;

import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.Registration.RegistrationHandler;

import java.io.InputStream;
import java.io.OutputStream;
import java.net.SocketException;
import java.util.logging.Logger;

public class StateHandler {
    public Status State = Status.Init;
    private final PacketHandler PacketHandler;
    private final Logger Log;
    private final OutputStream SocketOutput;
    private final InputStream SocketReceive;
    public final RegistrationHandler RegistrationHandler;

    public StateHandler(PacketHandler packetHandler, OutputStream socketOutput, InputStream socketReceive, Logger log) {
        PacketHandler = packetHandler;
        SocketOutput = socketOutput;
        SocketReceive = socketReceive;
        Log = log;

        RegistrationHandler = new RegistrationHandler(packetHandler, this, log);
    }

    public void StartReceiver() {
        synchronized (SocketReceive) {
            while(true) {
                try {
                    SocketReceive.wait(1);

                    Packet packet = PacketHandler.ReceivePacketBlocking();

                    if (packet == null) {
                        continue;
                    }

                    Log.warning("Packet received ID: " + packet.id());

                    switch (packet.id()) {
                        case 1 -> RegistrationHandler.HandleResponse(packet);
                        default -> {
                            Log.severe("UNKNOWN Packet Id received: " + packet.id());
                            break;
                        }
                    }

                } catch (Exception ex) {
                    if (ex instanceof SocketException) {
                        State = Status.Crash;
                        Log.severe("Socket connection to governor lost");

                        return;
                    }
                }
            }
        }
    }
}
