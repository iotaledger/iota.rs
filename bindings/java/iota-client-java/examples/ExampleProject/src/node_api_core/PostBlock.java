package node_api_core;

import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.TaggedDataPayload;
import org.iota.types.ids.BlockId;

public class PostBlock {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.testnet.shimmer.network";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Set up a block.
        Block b = ExampleUtils.setUpBlock(client);

        // Post the block.
        BlockId id = client.postBlock(b);

        // Print the id of the created block.
        System.out.println(id);
    }

}
