package tech.strateim.barium.Master.State.Proxy;

import generic.ProxyMessage;
import tech.strateim.barium.Game.ServerState;
import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;
import tech.strateim.barium.Master.State.AbstractState;
import tech.strateim.barium.Master.State.Proxy.Commands.WarnCommand;

import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.logging.Logger;

public class ProxyHandler extends AbstractState {

    private final HashMap<Integer, AbstractState> commandHandlers = new HashMap<>();

    public ProxyHandler(int id, PacketHandler packetHandler, Logger log, ServerState serverState) {
        super(id, packetHandler, log);

        commandHandlers.put(0, new WarnCommand(0, packetHandler, log, serverState));
    }

    @Override
    public void HandleResponse(Packet packet) throws Exception {
        ProxyMessage proxyMessage = (ProxyMessage) packetHandler.SerializePacket(ProxyMessage.getDefaultInstance(), packet);

        if (proxyMessage == null) {
            return;
        }

        if (commandHandlers.containsKey(proxyMessage.getMessageID())) {
            Packet decoded_packet = new Packet(StandardCharsets.ISO_8859_1.encode(proxyMessage.getMessageData()).array(), (short) proxyMessage.getMessageID());

            var handler = commandHandlers.get(proxyMessage.getMessageID());

            if (handler != null) {
                handler.HandleResponse(decoded_packet);
            }
        } else {
            log.warning("Unknown proxied command id received from anticheat, (" + proxyMessage.getMessageID() + ")");
        }
    }
}
