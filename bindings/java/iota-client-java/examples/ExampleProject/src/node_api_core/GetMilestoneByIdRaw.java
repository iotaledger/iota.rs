package node_api_core;

import org.apache.commons.codec.binary.Hex;
import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.MilestonePayload;
import org.iota.types.ids.MilestoneId;

public class GetMilestoneByIdRaw {

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig("{ \"nodes\": [ \"https://api.testnet.shimmer.network\" ], \"nodeSyncEnabled\": true }"));

        // Set up a milestone id for this example.
        MilestoneId milestoneId = ExampleUtils.setUpMilestoneId(client);

        // Get the milestone.
        byte[] milestoneBytes = client.getMilestoneByIdRaw(milestoneId);

        // Print the bytes
        System.out.println(Hex.encodeHex(milestoneBytes));
    }

}
