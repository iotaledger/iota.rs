import org.apache.commons.codec.binary.Hex;
import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.MilestoneId;

public class GetMilestoneByIdRaw {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a milestone ID for this example.
        MilestoneId milestoneId = ExampleUtils.setUpMilestoneId(client);

        // Get the milestone bytes.
        byte[] milestoneBytes = client.getMilestoneByIdRaw(milestoneId);

        // Print the milestone bytes.
        System.out.println(Hex.encodeHex(milestoneBytes));
    }
}