package output_builder;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.ids.NftId;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;

import java.util.ArrayList;
import java.util.List;

public class BuildNftOutput {

    private static final String DEFAULT_TESTNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // Generate the address
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String hexAddress = client.bech32ToHex(client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(0, 1))[0]);

        // Configure a simple NFT output.
        NftId nftId = new NftId("0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3");

        List<UnlockCondition> unlockConditions = new ArrayList<>();
        unlockConditions.add(new UnlockCondition("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"));

        NftOutputBuilderParams params = new NftOutputBuilderParams(
                null,
                null,
                nftId,
                unlockConditions,
                null,
                null
        );

        // Build the output.
        Output output = client.buildNftOutput(params);

        // Print the output.
        System.out.println(output.toString());

    }

}
