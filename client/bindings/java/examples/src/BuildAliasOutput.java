import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.Feature;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.AliasId;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;
import com.google.gson.GsonBuilder;

public class BuildAliasOutput {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Configure a simple Alias output.
        String hexAddress = client.bech32ToHex("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy");
        AliasId aliasId = new AliasId("0x0000000000000000000000000000000000000000000000000000000000000000");
        UnlockCondition[] unlockConditions = new UnlockCondition[]{
                new UnlockCondition("{ type: 4, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                new UnlockCondition("{ type: 5, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
        };
        Feature[] features = new Feature[]{
                // sender feature
                new Feature("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                // metadata feature, `hello` hex encoded
                new Feature("{ type: 2, data: \"0x68656c6c6f\" }")
        };
        Feature[] immutableFeatures = new Feature[]{
                // issuer feature
                new Feature("{ type: 1, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                // metadata feature, `hello` hex encoded
                new Feature("{ type: 2, data: \"0x68656c6c6f\" }")
        };
        AliasOutputBuilderParams params = new AliasOutputBuilderParams()
                .withAliasId(aliasId)
                .withStateMetadata("0x68656c6c6f")
                .withUnlockConditions(unlockConditions)
                .withFeatures(features)
                .withImmutableFeatures(immutableFeatures);

        // Build the output.
        Output output = client.buildAliasOutput(params);

        // Print the output.
        System.out.println(new GsonBuilder().setPrettyPrinting().create().toJson(output));

    }
}