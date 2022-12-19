import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.MilestoneId;
import org.iota.types.ids.OutputId;
import org.iota.types.responses.UtxoChangesResponse;

public class GetUtxoChangesById {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a milestone ID for this example.
        MilestoneId milestoneId = ExampleUtils.setUpMilestoneId(client);

        // Get the UTXO changes.
        UtxoChangesResponse response = client.getUtxoChangesById(milestoneId);

        // Print the milestone index.
        System.out.println(response.getIndex());

        // Print the created outputs.
        for (OutputId outputId : response.getCreatedOutputs())
            System.out.println(outputId);

        // Print the consumed outputs.
        for (OutputId outputId : response.getConsumedOutputs())
            System.out.println(outputId);
    }
}