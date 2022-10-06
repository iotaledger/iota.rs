package output_builder;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.output_builder.BasicOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;

public class BuildBasicOutput {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Configure a simple Basic output.
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String hexAddress = client.bech32ToHex(client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0]);
        String amount = "1000000";
        UnlockCondition[] unlockConditions = new UnlockCondition[]{new UnlockCondition("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\"} }")};
        BasicOutputBuilderParams params = new BasicOutputBuilderParams().withAmount(amount).withUnlockConditions(unlockConditions);

        // Build the output.
        Output output = client.buildBasicOutput(params);

        // Print the output.
        System.out.println(output.toString());

    }
}