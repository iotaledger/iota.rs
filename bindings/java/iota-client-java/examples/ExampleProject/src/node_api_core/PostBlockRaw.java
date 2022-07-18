package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.ids.BlockId;

public class PostBlockRaw {

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig("{ \"nodes\": [ \"https://api.testnet.shimmer.network\" ], \"nodeSyncEnabled\": true }"));

        // Set up a block.
        byte[] blockBytes = ExampleUtils.setUpBlockRaw(client);

        // Post the block.
        BlockId id = client.postBlockRaw(blockBytes);

        // Print the id of the created block.
        System.out.println(id);
    }

}
