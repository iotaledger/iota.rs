import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;

public class GetHealth {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the health of the given node.
        boolean health = client.getHealth("https://api.testnet.shimmer.network");

        // Print the response.
        System.out.println(health);
    }
}