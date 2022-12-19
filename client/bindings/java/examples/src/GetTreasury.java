import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.responses.TreasuryResponse;

public class GetTreasury {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the treasury.
        TreasuryResponse response = client.getTreasury();

        // Print the amount.
        System.out.println(response.getAmount());

        // Print the milestone ID.
        System.out.println(response.getMilestoneId());
    }
}