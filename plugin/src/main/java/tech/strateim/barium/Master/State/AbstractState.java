package tech.strateim.barium.Master.State;

import tech.strateim.barium.Master.Packet.Packet;
import tech.strateim.barium.Master.Packet.PacketHandler;

import java.util.logging.Logger;

public abstract class AbstractState {
    protected int id = -1;
    protected PacketHandler packetHandler;

    protected Logger log;

    public abstract void HandleResponse(Packet packet) throws Exception;

    public AbstractState(int id, PacketHandler packetHandler, Logger log) {
        this.id = id;
        this.packetHandler = packetHandler;
        this.log = log;
    }
}
