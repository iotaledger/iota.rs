import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.Output;
import org.iota.types.OutputMetadata;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.OutputId;

import java.util.Map;

public class GetOutputs {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a output ID for this example.
        OutputId outputId = ExampleUtils.setUpOutputId(client);
        
        // Get the output for the given output id.
        Map.Entry<Output, OutputMetadata> outputData = client.getOutput(outputId);

        // Print the output and its metadata.
        System.out.println(outputData.getKey());
        System.out.println(outputData.getValue());
    }
}