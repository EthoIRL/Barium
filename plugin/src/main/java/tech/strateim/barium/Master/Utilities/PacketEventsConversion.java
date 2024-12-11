package tech.strateim.barium.Master.Utilities;

import com.github.retrooper.packetevents.manager.server.SystemOS;
import generic.Os;

public class PacketEventsConversion {
    public static Os SystemConversion(SystemOS systemOS) {
        return switch (systemOS) {
            case LINUX -> Os.Linux;
            case WINDOWS -> Os.Windows;
            case MACOS -> Os.MacOS;
            case OTHER -> Os.Other;
        };
    }
}
