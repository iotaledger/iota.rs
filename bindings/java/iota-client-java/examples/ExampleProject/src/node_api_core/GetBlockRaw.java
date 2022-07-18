package node_api_core;

import org.apache.commons.codec.binary.Hex;
import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.ids.BlockId;

public class GetBlockRaw {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.testnet.shimmer.network";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Get a block id for which the block can be requested.
        BlockId blockId = ExampleUtils.setUpBlockId(client);

        // Get the block.
        byte[] blockBytes = client.getBlockRaw(blockId);

        // Print the bytes
        System.out.println(Hex.encodeHex(blockBytes));
    }

}
