package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Receipt;

public class GetReceiptsMigratedAt {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Setup some milestone index.
        System.out.println(client.getNodeInfo().getNodeInfo());
        int milestoneIndex = client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("confirmedMilestone").getAsJsonObject().get("index").getAsInt();

        // Get the receipts at the specific milestone index.
        Receipt[] receipts = client.getReceiptsMigratedAt(milestoneIndex);

        // Print the receipts.
        for(Receipt receipt: receipts)
            System.out.println(receipt);
    }

}
