import org.iota.Client;
import org.iota.types.BlockMetadata;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;

public class GetBlockMetadata {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a block ID for this example.
        BlockId blockId = ExampleUtils.setUpBlockId(client);

        // Get the block metadata.
        BlockMetadata blockMetadata = client.getBlockMetadata(blockId);

        // Print the block metadata.
        System.out.println(blockMetadata);
    }
}