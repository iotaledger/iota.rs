package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Receipt;
import org.iota.types.responses.TreasuryResponse;

public class GetTreasury {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Get the treasury.
        TreasuryResponse response = client.getTreasury();

        // Print the amount.
        System.out.println(response.getAmount());

        // Print the milestone id.
        System.out.println(response.getMilestoneId());
    }

}
