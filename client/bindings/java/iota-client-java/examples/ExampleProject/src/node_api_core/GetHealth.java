package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;

public class GetHealth {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the health of the given node.
        boolean health = client.getHealth("https://api.testnet.shimmer.network");

        // Print the response.
        System.out.println(health);
    }
}