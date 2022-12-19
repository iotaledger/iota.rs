import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.NftId;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;

public class BuildNftOutput {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Configure a simple NFT output.
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String hexAddress = client.bech32ToHex(client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0]);
        NftId nftId = new NftId("0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3");
        UnlockCondition[] unlockConditions = new UnlockCondition[]{new UnlockCondition("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")};
        NftOutputBuilderParams params = new NftOutputBuilderParams()
                .withNftId(nftId)
                .withUnlockConditions(unlockConditions);

        // Build the output.
        Output output = client.buildNftOutput(params);

        // Print the output.
        System.out.println(output.toString());

    }
}