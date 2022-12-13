import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;

public class GetBlock {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a block ID for this example.
        BlockId blockId = ExampleUtils.setUpBlockId(client);

        // Get the block.
        Block block = client.getBlock(blockId);

        // Print the block.
        System.out.println(block);
    }
}