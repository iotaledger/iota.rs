package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.MilestonePayload;
import org.iota.types.ids.MilestoneId;
import org.iota.types.ids.OutputId;
import org.iota.types.responses.UtxoChangesResponse;

public class GetUtxoChangesById {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Set up a milestone id.
        MilestoneId milestoneId = new MilestoneId(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("confirmedMilestone").getAsJsonObject().get("milestoneId").getAsString());

        // Get the UTXO changes.
        UtxoChangesResponse response = client.getUtxoChangesById(milestoneId);

        // Print the milestone index.
        System.out.println(response.getIndex());

        // Print the created outputs.
        for(OutputId outputId: response.getCreatedOutputs())
            System.out.println(outputId);

        // Print the consumed outputs.
        for(OutputId outputId: response.getConsumedOutputs())
            System.out.println(outputId);
    }

}
