import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.responses.NodeInfoResponse;

public class GetInfo {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the node information for a given node.
        NodeInfoResponse response = client.getNodeInfo();

        // Print the URL of the node that was requested.
        System.out.println(response.getNodeUrl());

        // Print the node information for the requested node.
        System.out.println(response.getNodeInfo());
    }
}