package output_builder;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.ids.AliasId;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;

import java.util.ArrayList;
import java.util.List;

public class BuildAliasOutput {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Generate the address
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String hexAddress = client.bech32ToHex(client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(0, 1))[0]);

        // Configure a simple alias output.
        AliasId aliasId = new AliasId("0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa");
        UnlockCondition[] unlockConditions = new UnlockCondition[] {
                new UnlockCondition("{ type: 4, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                new UnlockCondition("{ type: 5, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
        };
        AliasOutputBuilderParams params = new AliasOutputBuilderParams(
                null,
                null,
                aliasId,
                null,
                null,
                null,
                unlockConditions,
                null,
                null
        );

        // Build the output.
        Output output = client.buildAliasOutput(params);

        // Print the output.
        System.out.println(output.toString());

    }
}