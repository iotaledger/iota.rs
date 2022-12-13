import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;

public class GetTips {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the tips.
        BlockId[] tips = client.getTips();

        // Print the tips.
        for (BlockId id : tips)
            System.out.println(id);
    }
}