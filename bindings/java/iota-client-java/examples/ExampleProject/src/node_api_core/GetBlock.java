package node_api_core;

import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.ids.BlockId;

public class GetBlock {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig("{ \"nodes\": [ \"https://api.testnet.shimmer.network\" ], \"nodeSyncEnabled\": true }"));

        // Set up a block for this example.
        BlockId blockId = ExampleUtils.setUpBlockId(client);

        // Get the block.
        Block block = client.getBlock(blockId);

        // Print the block.
        System.out.println(block);
    }
}