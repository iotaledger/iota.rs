package node_api_core;

import org.iota.Client;
import org.iota.apis.NodeIndexerApi;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Output;
import org.iota.types.OutputMetadata;
import org.iota.types.ids.OutputId;

import java.util.Map;

public class GetOutputs {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Get the output for the given output id.
        Map.Entry<Output, OutputMetadata> outputData = client.getOutput(new OutputId("..."));

        // Print the output and its metadata.
        System.out.println(outputData.getKey());
        System.out.println(outputData.getValue());
    }
}
