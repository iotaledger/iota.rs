import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.Feature;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.output_builder.BasicOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;
import com.google.gson.*;

public class BuildBasicOutput {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        String hexAddress = client.bech32ToHex("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy");
        String amount = "1000000";
        UnlockCondition addressUnlockCondition = new UnlockCondition("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\"} }");

        // Build most basic output with amound and a single address unlock condition
        BasicOutputBuilderParams basicParams = new BasicOutputBuilderParams()
            .withAmount(amount)
            .withUnlockConditions(new UnlockCondition[]{addressUnlockCondition});

        Output basicOutput = client.buildBasicOutput(basicParams);

        System.out.println(
            new GsonBuilder().setPrettyPrinting().create().toJson(JsonParser.parseString(basicOutput.toString()))
        );

        // Output with metadata feature block
        BasicOutputBuilderParams metadataParams = new BasicOutputBuilderParams()
            .withAmount(amount)
            .withUnlockConditions(new UnlockCondition[]{addressUnlockCondition})
            // "Hello, World!" hex encoded
            .withFeatures(new Feature[]{new Feature("{ type: 2, data: \"0x48656c6c6f2c20576f726c6421\" }")});

        Output metadataOutput = client.buildBasicOutput(metadataParams);

        System.out.println(
            new GsonBuilder().setPrettyPrinting().create().toJson(JsonParser.parseString(metadataOutput.toString()))
        );

        // Output with storage deposit return
        UnlockCondition storageReturnUnlock = new UnlockCondition(
            "{ type: 1, returnAddress: { type: 0, pubKeyHash: \"" + hexAddress + "\"}, amount: \"" + amount + "\"}"
        ); 
        BasicOutputBuilderParams storageParams = new BasicOutputBuilderParams()
            .withAmount(amount)
            .withUnlockConditions(new UnlockCondition[]{addressUnlockCondition, storageReturnUnlock});

        Output storageOutput = client.buildBasicOutput(storageParams);

        System.out.println(
            new GsonBuilder().setPrettyPrinting().create().toJson(JsonParser.parseString(storageOutput.toString()))
        );

        // Output with expiration
        UnlockCondition expirationUnlock = new UnlockCondition(
            "{ type: 3, returnAddress: { type: 0, pubKeyHash: \"" + hexAddress + "\"}, unixTime: 1 }"
        );
        BasicOutputBuilderParams expirationParams = new BasicOutputBuilderParams()
            .withAmount(amount)
            .withUnlockConditions(new UnlockCondition[]{addressUnlockCondition, expirationUnlock});

        Output expirationOutput = client.buildBasicOutput(expirationParams);

        System.out.println(
            new GsonBuilder().setPrettyPrinting().create().toJson(JsonParser.parseString(expirationOutput.toString()))
        );

        // Output with timelock
        UnlockCondition timeUnlock = new UnlockCondition(
            "{ type: 2, unixTime: 1 }"
        );
        BasicOutputBuilderParams timelockParams = new BasicOutputBuilderParams()
            .withAmount(amount)
            .withUnlockConditions(new UnlockCondition[]{addressUnlockCondition, timeUnlock});

        Output timelockOutput = client.buildBasicOutput(timelockParams);

        System.out.println(
            new GsonBuilder().setPrettyPrinting().create().toJson(JsonParser.parseString(timelockOutput.toString()))
        );
    }
}