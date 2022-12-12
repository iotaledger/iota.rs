import org.apache.commons.codec.binary.Hex;
import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;

public class GetBlockRaw {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a block ID for this example.
        BlockId blockId = ExampleUtils.setUpBlockId(client);

        // Get the block bytes.
        byte[] blockBytes = client.getBlockRaw(blockId);

        // Print the block bytes.
        System.out.println(Hex.encodeHex(blockBytes));
    }
}