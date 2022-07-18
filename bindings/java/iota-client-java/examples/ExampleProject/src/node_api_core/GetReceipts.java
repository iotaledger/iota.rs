package node_api_core;

import org.iota.Client;
import org.iota.types.*;
import org.iota.types.ids.BlockId;

public class GetReceipts {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.testnet.shimmer.network";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Get the receipts.
        Receipt[] receipts = client.getReceipts();

        // Print the receipts.
        for(Receipt receipt: receipts)
            System.out.println(receipt);
    }

}
