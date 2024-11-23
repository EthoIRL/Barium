package tech.strateim.barium.Master;

import init.Os;
import init.Protocol;
import init.Register;
import init.RegistrationResponse;
import org.bukkit.Server;
import tech.strateim.barium.Master.Enum.Status;
import tech.strateim.barium.Master.State.PacketHandler;

import javax.annotation.Nullable;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.logging.Level;
import java.util.logging.Logger;

public class Remote {
    private static Logger Log;
    private Socket Socket;
    private OutputStream SocketOutput;
    private InputStream SocketReceive;
    public final ExecutorService ExecutorService = Executors.newFixedThreadPool(8);
    public Status State = Status.Init;

    private PacketHandler PacketHandler;

    public Remote(Logger log) {
        Log = log;
    }

    public @Nullable PacketHandler Start(String governorAddress, int governorPort, Server localServer) {
        while (true) {
            try {
                Socket = new Socket(governorAddress, governorPort);
                Log.info("Connected to governor server");

                break;
            } catch (Exception ex) {
                Log.severe("Failed to connect to governor server");

                try {
                    Thread.sleep(5000);
                } catch (Exception ignore) {}
            }
        }

        try {
            SocketOutput = Socket.getOutputStream();
        } catch (Exception ex) {
            Log.log(Level.SEVERE, "Failed to get sender socket stream to governor server");
            State = Status.Crash;

            return null;
        }

        try {
            SocketReceive = Socket.getInputStream();
        } catch (Exception ex) {
            Log.log(Level.SEVERE, "Failed to get receiver socket stream to governor server");
            State = Status.Crash;

            return null;
        }

        PacketHandler = new PacketHandler(Log, SocketOutput, SocketReceive);

        ExecutorService.execute(this::StartGovernorReceiver);
        ExecutorService.execute(() -> InitRegister(localServer));

        return PacketHandler;
    }

    private void InitRegister(Server localServer) {
        Protocol.Builder protocolBuilder = Protocol.newBuilder()
                .setGeyser(false)
                .setViaBackwards(false)
                .setViaRewind(false)
                .setViaVersion(false);

        Register register = Register.newBuilder()
                .setPluginVersion(0)
                .setServerVersion(134)
                .setOs(Os.Windows)
                .setProtocol(protocolBuilder)
                .build();

        PacketHandler.SendPacket(register, 0);
    }


    private void StartGovernorReceiver() {
        while(true) {
            try {
                synchronized (SocketReceive) {
                    SocketReceive.wait(1);

                    switch (State) {
                        case Init -> {
                            RegistrationResponse response = (RegistrationResponse) PacketHandler.ReceivePacketBlocking(RegistrationResponse.getDefaultInstance(), 1);

                            if (response == null) {
                                continue;
                            }

                            if (response.getSucceeded()) {
                                Log.info(response.toString());

                                State = Status.Ready;
                                continue;
                            }

                            Log.severe("Failed to authenticate, registration failed");
                            Log.info(response.toString());

                            State = Status.Shutdown;

                            break;
                        }
                        case null, default -> {
                            break;
                        }
                    }
                }
            } catch (Exception ex) {
                Log.severe("ERROR " + ex.toString());
            }
        }
    }
}