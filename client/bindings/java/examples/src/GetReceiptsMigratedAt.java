import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.Receipt;
import org.iota.types.expections.InitializeClientException;

public class GetReceiptsMigratedAt {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a milestone index for this example.
        int milestoneIndex = ExampleUtils.setUpMilestoneIndex(client);

        // Get the receipts at the specific milestone index.
        Receipt[] receipts = client.getReceiptsMigratedAt(milestoneIndex);

        // Print the receipts.
        for (Receipt receipt : receipts)
            System.out.println(receipt);
    }
}