package tech.strateim.barium.Master.State;

import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.Remote;
import tech.strateim.barium.Master.State.Disconnection.DisconnectionHandler;
import tech.strateim.barium.Master.State.Registration.RegistrationHandler;

import java.io.InputStream;
import java.io.OutputStream;
import java.net.SocketException;
import java.util.HashMap;
import java.util.logging.Logger;

public class StateHandler {
    public Status State = Status.Initialization;
    public String Key = null;
    private final PacketHandler PacketHandler;
    private final Logger Log;
    private final OutputStream SocketOutput;
    private final InputStream SocketReceive;
    private final HashMap<Integer, AbstractState> stateHandlers = new HashMap<>();

    public StateHandler(PacketHandler packetHandler, OutputStream socketOutput, InputStream socketReceive, Logger log, Remote remote) {
        PacketHandler = packetHandler;
        SocketOutput = socketOutput;
        SocketReceive = socketReceive;
        Log = log;

        stateHandlers.put(1, new RegistrationHandler(1, packetHandler, log, this));
        stateHandlers.put(2, new DisconnectionHandler(2, packetHandler, log, remote, this));
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

                    if (!stateHandlers.containsKey((int)packet.id())) {
                        Log.severe("Packet ID does not exist within stateHandlers!");

                        continue;
                    }

                    stateHandlers.get((int)packet.id()).HandleResponse(packet);
                } catch (Exception ex) {
                    if (ex instanceof SocketException) {
                        State = Status.Crash;
                        Log.severe("Socket connection to governor lost");

                        return;
                    } else {
                        Log.warning("Exception occurred when handling a packet (" + ex + ")");

                        continue;
                    }
                }
            }
        }
    }
}
