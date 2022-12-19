import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;

public class GenerateMnemonic {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Generate a mnemonic.
        String mnemonic = client.generateMnemonic();

        // Print the mnemonic.
        System.out.println(mnemonic);
    }
}