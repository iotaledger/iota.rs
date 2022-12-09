import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.MilestonePayload;
import org.iota.types.expections.InitializeClientException;

public class GetMilestoneByIndex {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a milestone index for this example.
        int milestoneIndex = ExampleUtils.setUpMilestoneIndex(client);

        // Get the milestone.
        MilestonePayload milestone = client.getMilestoneByIndex(milestoneIndex);

        // Print the milestone.
        System.out.println(milestone);
    }
}