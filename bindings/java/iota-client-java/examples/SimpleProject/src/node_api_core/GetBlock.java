package node_api_core;

import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.TaggedDataPayload;
import org.iota.types.ids.BlockId;

public class GetBlock {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Set up a block to later get it.
        Block b = client.submitBlockPayload(new TaggedDataPayload("{\"type\":5,\"tag\":\"0x68656c6c6f20776f726c64\",\"data\":\"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\"}"));
        BlockId blockId = client.postBlock(b);

        // Get the block.
        Block block = client.getBlock(blockId);

        // Print the block.
        System.out.println(block);
    }

}
