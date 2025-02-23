package tech.strateim.barium.Listeners.Handlers;

import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.protocol.player.ClientVersion;
import com.github.retrooper.packetevents.protocol.world.Location;
import com.github.retrooper.packetevents.wrapper.play.client.WrapperPlayClientPlayerFlying;
import movement.Px_PlayerMovement;

import javax.annotation.Nullable;

import static com.github.retrooper.packetevents.protocol.packettype.PacketType.Play.Client.*;

public class MovementHandler {

    @Nullable
    public static GenericPacket HandlePosition(String userUuid, PacketReceiveEvent event) {
        WrapperPlayClientPlayerFlying flying = new WrapperPlayClientPlayerFlying(event);

        if (flying.hasPositionChanged() || flying.hasRotationChanged()) {
            Location location = flying.getLocation();

            Px_PlayerMovement playerMovement = Px_PlayerMovement.newBuilder()
                    .setUuid(userUuid)
                    .setX(location.getX())
                    .setY(location.getY())
                    .setZ(location.getZ())
                    .setGround(flying.isOnGround())
                    .build();

            return new GenericPacket(playerMovement, 3);
        }

        return null;
    }

    public static boolean IsPosition(int id, ClientVersion clientVersion) {
        return id == PLAYER_POSITION.getId(clientVersion) ||
                id == PLAYER_POSITION_AND_ROTATION.getId(clientVersion) ||
                id == PLAYER_FLYING.getId(clientVersion);
    }
}
