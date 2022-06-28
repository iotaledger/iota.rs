package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.responses.NodeInfoResponse;

public class GetInfo {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Get the node information for a given node.
        NodeInfoResponse response = client.getNodeInfo();

        // Print the URL of the node that was requested.
        System.out.println(response.getNodeUrl());

        // Print the node information for the requested node.
        System.out.println(response.getNodeInfo());
    }
}
