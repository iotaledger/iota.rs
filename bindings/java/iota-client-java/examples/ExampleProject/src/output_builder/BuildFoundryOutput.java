package output_builder;

import org.iota.Client;
import org.iota.types.*;
import org.iota.types.ids.AliasId;
import org.iota.types.output_builder.FoundryOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;

import java.util.ArrayList;
import java.util.List;

public class BuildFoundryOutput {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Generate the address
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String hexAddress = client.bech32ToHex(client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(0, 1))[0]);

        // Configure a simple foundry output.
        AliasId aliasId = new AliasId("0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa");

        int serialNumber = 1;

        List<NativeToken> nativeTokens = new ArrayList<>();
        nativeTokens.add(new NativeToken("{ id: '0x081e6439529b020328c08224b43172f282cb16649d50c891fa156365323667e47a0100000000', amount: '0x32' }"));

        TokenScheme tokenScheme = new TokenScheme("{ type: 0, meltedTokens: '0x0', mintedTokens: '0x32', maximumSupply: '0x64' }");

        List<UnlockCondition> unlockConditions = new ArrayList<>();
        unlockConditions.add(new UnlockCondition("{ type: 6, address: { type: 8, aliasId: " + aliasId + "  } }"));

        FoundryOutputBuilderParams params = new FoundryOutputBuilderParams(
                null,
                nativeTokens,
                serialNumber,
                tokenScheme,
                unlockConditions,
                null,
                null
        );

        // Build the output.
        Output output = client.buildFoundryOutput(params);

        // Print the output.
        System.out.println(output.toString());

    }

}
