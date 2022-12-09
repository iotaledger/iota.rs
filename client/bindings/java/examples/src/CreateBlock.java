import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;

import java.util.Map;

public class CreateBlock {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Create the most simple block.
        Map.Entry<BlockId, Block> b = client.buildAndPostBlock(null, null);

        // Print the block.
        System.out.println(b);
    }
}