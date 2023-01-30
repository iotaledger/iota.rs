import org.iota.Client;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;
import org.iota.types.secret.*;

import java.util.Map;

public class PrepareAndSignTransaction {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Build the secret manager.
        // NOTE: `YOUR_SECRET_MNEMONIC` serves as a placeholder. Replace it with the mnemonic you want to use to sign the transaction.
        SecretManager secretManager = new MnemonicSecretManager("YOUR_SECRET_MNEMONIC");

        // Build the output for the transaction.
        String receiverAddress = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0];
        BuildBlockOptions.ClientBlockBuilderOutputAddress output = new BuildBlockOptions.ClientBlockBuilderOutputAddress(receiverAddress, Integer.toString(1000000));

        // Build block.
        Map.Entry<BlockId, Block> b = client.buildAndPostBlock(secretManager, new BuildBlockOptions());

        // Print the block ID.
        System.out.println(b.getKey());
    }
}