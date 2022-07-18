package node_api_core;

import com.google.gson.JsonObject;
import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.ids.BlockId;

public class CreateBlock {

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig("{ \"nodes\": [ \"https://api.testnet.shimmer.network\" ], \"nodeSyncEnabled\": true }"));

        // Set up the most simple block.
        Block b = client.generateBlock(null, null);

        // Print the block.
        System.out.println(b);
    }

}
