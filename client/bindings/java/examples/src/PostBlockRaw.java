import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;

public class PostBlockRaw {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a block for this example.
        byte[] blockBytes = ExampleUtils.setUpBlockRaw(client);

        // Post the block.
        BlockId id = client.postBlockRaw(blockBytes);

        // Print the id of the created block.
        System.out.println(id);
    }
}