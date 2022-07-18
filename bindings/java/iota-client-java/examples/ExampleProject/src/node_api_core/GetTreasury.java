package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.responses.TreasuryResponse;

public class GetTreasury {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig("{ \"nodes\": [ \"https://api.testnet.shimmer.network\" ], \"nodeSyncEnabled\": true }"));

        // Get the treasury.
        TreasuryResponse response = client.getTreasury();

        // Print the amount.
        System.out.println(response.getAmount());

        // Print the milestone id.
        System.out.println(response.getMilestoneId());
    }
}