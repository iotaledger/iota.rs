package output_builder;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.AliasId;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;

public class BuildAliasOutput {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Configure a simple Alias output.
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String hexAddress = client.bech32ToHex(client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0]);
        AliasId aliasId = new AliasId("0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa");
        UnlockCondition[] unlockConditions = new UnlockCondition[]{
                new UnlockCondition("{ type: 4, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                new UnlockCondition("{ type: 5, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
        };
        AliasOutputBuilderParams params = new AliasOutputBuilderParams()
                .withAliasId(aliasId)
                .withUnlockConditions(unlockConditions);

        // Build the output.
        Output output = client.buildAliasOutput(params);

        // Print the output.
        System.out.println(output.toString());

    }
}