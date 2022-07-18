package node_api_core;

import org.iota.Client;
import org.iota.types.*;
import org.iota.types.ids.BlockId;

public class GetBlockMetadata {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.testnet.shimmer.network";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Set up a block for this example.
        BlockId blockId = ExampleUtils.setUpBlockId(client);

        // Get the bytes of the block.
        BlockMetadata blockMetadata = client.getBlockMetadata(blockId);

        // Print the block metadata.
        System.out.println(blockMetadata);
    }

}
