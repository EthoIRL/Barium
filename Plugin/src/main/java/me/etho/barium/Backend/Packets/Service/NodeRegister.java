package me.etho.barium.Backend.Packets.Service;

public class NodeRegister {
    public String server_ip;
    public String server_version;
    public Region server_region;
    public String server_os;
    public boolean via_version;

    public enum Region {
        NA, // North America
        EU, // Europe
        AS, // Asia
        AF, // Africa
        OC, // Oceania
        SA, // South America
        AN // Antarctica WTF?
    }
}
