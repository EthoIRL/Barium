package tech.strateim.barium.Master.State;

import tech.strateim.barium.Master.Packet.Packet;

public abstract class AbstractState {
    protected int id = -1;

    public abstract void HandleResponse(Packet packet) throws Exception;

    public AbstractState(int id) {
        this.id = id;
    }
}
