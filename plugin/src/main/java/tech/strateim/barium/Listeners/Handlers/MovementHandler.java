package tech.strateim.barium.Listeners.Handlers;

import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.protocol.player.ClientVersion;
import com.github.retrooper.packetevents.protocol.world.Location;
import com.github.retrooper.packetevents.wrapper.play.client.WrapperPlayClientPlayerFlying;
import movement.Px_PlayerGround;
import movement.Px_PlayerMovement;
import movement.Px_PlayerRotation;
import tech.strateim.barium.Listeners.NetworkListener;

import static com.github.retrooper.packetevents.protocol.packettype.PacketType.Play.Client.*;

public class MovementHandler {

    public static void HandlePosition(String userUuid, PacketReceiveEvent event, NetworkListener networkListener) {
        WrapperPlayClientPlayerFlying flying = new WrapperPlayClientPlayerFlying(event);
        Location location = flying.getLocation();

        Px_PlayerGround playerGround = Px_PlayerGround.newBuilder()
                .setUuid(userUuid)
                .setGround(flying.isOnGround())
                .build();

        networkListener.HandlePacket(playerGround, 5);

        if (flying.hasPositionChanged()) {
            Px_PlayerMovement playerMovement = Px_PlayerMovement.newBuilder()
                    .setUuid(userUuid)
                    .setX(location.getX())
                    .setY(location.getY())
                    .setZ(location.getZ())
                    .build();

            networkListener.HandlePacket(playerMovement, 3);
        }

    }

    public static boolean IsPosition(int id, ClientVersion clientVersion) {
        return id == PLAYER_POSITION.getId(clientVersion) ||
                id == PLAYER_POSITION_AND_ROTATION.getId(clientVersion) ||
                id == PLAYER_FLYING.getId(clientVersion);
    }
}
