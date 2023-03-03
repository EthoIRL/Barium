package me.etho.barium.Backend.Packets;

import lombok.Getter;

@Getter
public final class Packet<T> {
    private final int id;
    private final T packet;
    public Packet(int id, T packet) {
        this.id = id;
        this.packet = packet;
    }
}
