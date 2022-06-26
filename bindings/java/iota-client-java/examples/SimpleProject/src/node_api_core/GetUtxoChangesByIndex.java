package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.ids.MilestoneId;
import org.iota.types.ids.OutputId;
import org.iota.types.responses.UtxoChangesResponse;

public class GetUtxoChangesByIndex {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Set up a milestone index.
        int milestoneIndex = client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("confirmedMilestone").getAsJsonObject().get("index").getAsInt();

        // Get the UTXO changes.
        UtxoChangesResponse response = client.getUtxoChangesByIndex(milestoneIndex);

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
