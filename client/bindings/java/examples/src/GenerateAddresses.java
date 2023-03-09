import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;

public class GenerateAddresses {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        
        // Generate public address with default account index and range.
        String[] defaultAddresses = client.generateAddresses(secretManager, new GenerateAddressesOptions());

        // Print the addresses.
        System.out.println("List of generated public addresses:");
        for (String address : defaultAddresses) {
            System.out.println(address);
        }
        System.out.println();

        // Generate public address with custom account index and range.
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withAccountIndex(0).withRange(new Range(0, 4)));

        // Print the addresses.
        System.out.println("List of generated public addresses:");
        for (String address : addresses) {
            System.out.println(address);
        }
        System.out.println();

        // Generate internal addresses with custom account index and range.
        String[] internalAddresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withInternal(true).withAccountIndex(0).withRange(new Range(0, 4)));

        // Print the addresses.
        System.out.println("List of generated internal addresses:");
        for (String address : internalAddresses) {
            System.out.println(address);
        }
        System.out.println();

        // Generate addresses with providing all inputs, that way it can also be done offline without a node.
        String[] offlineGeneratedAddresses = client.generateAddresses(secretManager, 
            new GenerateAddressesOptions()
            .withCoinType(4219)
            .withAccountIndex(0)
            .withRange(new Range(0, 4))
            .withInternal(false)
            // Generating addresses with client.generateAddresses(secretManager, new GenerateAddressesOptions()), will by default get the bech32_hrp (Bech32
            // human readable part) from the node info, generating it "offline" requires setting it in the generateAddressesOptions
            .withBech32Hrp("rms"));

        // Print the addresses.
        System.out.println("List of offline generated public addresses:");
        for (String address : offlineGeneratedAddresses) {
            System.out.println(address);
        }
        System.out.println();
    }
}
