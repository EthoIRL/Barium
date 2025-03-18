package tech.strateim.barium.Listeners.Handlers;

import collision.Px_PlayerCollision;
import com.github.retrooper.packetevents.event.PacketReceiveEvent;
import com.github.retrooper.packetevents.protocol.packettype.PacketType;
import com.github.retrooper.packetevents.protocol.player.ClientVersion;
import com.github.retrooper.packetevents.wrapper.play.client.WrapperPlayClientPlayerFlying;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.util.BoundingBox;
import tech.strateim.barium.Game.Clients.Client;
import tech.strateim.barium.Listeners.NetworkListener;

import java.util.LinkedList;
import java.util.List;

public class CollisionHandler {

    public static void HandleCollision(String userUuid, PacketReceiveEvent event, NetworkListener networkListener, Client client) {
        WrapperPlayClientPlayerFlying position = new WrapperPlayClientPlayerFlying(event);

        // Assuming Feet Y
        var y = position.getLocation().getY();

        var playerBounding = client.getPlayer().getBoundingBox();
        var world = client.getPlayer().getWorld();

        var downwardBlocks = getBlocks(world, playerBounding.expand(0, 0.05, 0));

        var solidGround = downwardBlocks.stream()
                .anyMatch(block -> block.getType().isSolid());
        var blockAbove = downwardBlocks.stream()
                .anyMatch(block -> block.getLocation().getY() - y >= 1.0);
        var inWeb = downwardBlocks.stream()
                .anyMatch(block -> block.getType() == Material.COBWEB);
        var inLiquid = downwardBlocks.stream()
                .anyMatch(block -> block.getType() == Material.WATER || block.getType() == Material.LAVA);

        if (solidGround || blockAbove || inWeb || inLiquid) {
            Px_PlayerCollision playerCollision = Px_PlayerCollision.newBuilder()
                    .setUuid(userUuid)
                    .setBlockBelow(solidGround)
                    .setBlockAbove(blockAbove)
                    .setInWeb(inWeb)
                    .setInLiquid(inLiquid)
                    .build();
            networkListener.HandlePacket(playerCollision, 6);
        }

        downwardBlocks.clear();
    }

    public static boolean IsClientCollision(int id, ClientVersion clientVersion) {
        return id == PacketType.Play.Client.PLAYER_POSITION.getId(clientVersion) ||
                id == PacketType.Play.Client.PLAYER_POSITION_AND_ROTATION.getId(clientVersion) ||
                id == PacketType.Play.Client.PLAYER_FLYING.getId(clientVersion);
    }

    public static List<Block> getBlocks(final World world, BoundingBox boundingBox) {
        final List<Block> blockList = new LinkedList<>();

        final int minX = (int) Math.floor(boundingBox.getMinX());
        final int maxX = (int) Math.ceil(boundingBox.getMaxX());

        final int minY = (int) Math.floor(boundingBox.getMinY());
        final int maxY = (int) Math.ceil(boundingBox.getMaxY());

        final int minZ = (int) Math.floor(boundingBox.getMinZ());
        final int maxZ = (int) Math.ceil(boundingBox.getMaxZ());

        for (int i = minX; i < maxX; ++i) {
            for (int j = minY; j < maxY; ++j) {
                for (int k = minZ; k < maxZ; ++k) {
                    var block = new Location(world, i, j, k).getBlock();

                    if (block.getType() == Material.AIR) {
                        continue;
                    }

                    blockList.add(block);
                }
            }
        }
        return blockList;
    }


}
