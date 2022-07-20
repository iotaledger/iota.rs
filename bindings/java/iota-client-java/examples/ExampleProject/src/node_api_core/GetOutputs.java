package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Output;
import org.iota.types.OutputMetadata;
import org.iota.types.ids.OutputId;

import java.util.Map;

import static node_api_core.ExampleUtils.setUpOutputId;

public class GetOutputs {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a output ID for this example.
        OutputId outputId = setUpOutputId(client);
        
        // Get the output for the given output id.
        Map.Entry<Output, OutputMetadata> outputData = client.getOutput(outputId);

        // Print the output and its metadata.
        System.out.println(outputData.getKey());
        System.out.println(outputData.getValue());
    }
}