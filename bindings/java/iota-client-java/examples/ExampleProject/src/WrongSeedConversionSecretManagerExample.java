import com.google.gson.JsonObject;
import org.iota.Client;
import org.iota.apis.NodeIndexerApi;
import org.iota.types.*;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.OutputId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.GenerateBlockOptions;
import org.iota.types.secret.SeedSecretManager;
import org.iota.types.secret.WrongSeedConversionSecretManager;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/*
An incorrect seed conversion from Java to Rust in February 2022 resulted in incorrectly derived addresses. See https://github.com/iotaledger/iota.rs/pull/800 for more details.
This example shows how to access and migrate the funds located on the incorrectly derived addresses.
This example will try to migrate funds from the first 50 addresses of the seed.
 */
public class WrongSeedConversionSecretManagerExample {

    private static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    private static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_TESTNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(config);

        // The hex seed that is affected by the seed conversion bug.
        String hexSeed = "";
        WrongSeedConversionSecretManager wrongSecretManager = new WrongSeedConversionSecretManager(hexSeed);

        // Generate the first 50 affected addresses of account index 0.
        GenerateAddressesOptions addressesOptions = new GenerateAddressesOptions().withAccountIndex(0).withRange(1, 50);
        String[] affectedAddresses = client.generateAddresses(wrongSecretManager, addressesOptions);

        System.out.println(affectedAddresses[0]);

        // Get the affected outputs
        List<OutputId> affectedOutputIds = new ArrayList<>();
        for (String address : affectedAddresses) {
            OutputId[] outputIds = client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address));
            affectedOutputIds.addAll(List.of(outputIds));
        }

        if (affectedOutputIds.size() == 0) {
            throw new RuntimeException("cannot find any outputs to migrate");
        }

        // Prepare the inputs for the transaction
        List<UtxoInput> inputs = new ArrayList<>();
        int amountToMigrate = 0;
        for (Map.Entry<Output, OutputMetadata> e : client.getOutputs(affectedOutputIds.toArray(new OutputId[affectedOutputIds.size()]))) {
            Output output = e.getKey();
            OutputMetadata metadata = e.getValue();

            JsonObject utxoInputJsonObject = new JsonObject();
            utxoInputJsonObject.addProperty("type", output.getJson().get("type").getAsInt());
            utxoInputJsonObject.addProperty("transactionId", metadata.getJson().get("transactionId").getAsString());
            utxoInputJsonObject.addProperty("transactionOutputIndex", metadata.getJson().get("outputIndex").getAsInt());
            inputs.add(new UtxoInput(utxoInputJsonObject));

            amountToMigrate += output.getJson().get("amount").getAsInt();
        }

        // Build the output for the transaction
        String receiverAddress = client.generateAddresses(new SeedSecretManager(hexSeed), new GenerateAddressesOptions().withRange(0, 1))[0];
        GenerateBlockOptions.ClientBlockBuilderOutputAddress output = new GenerateBlockOptions.ClientBlockBuilderOutputAddress(receiverAddress, Integer.toString(amountToMigrate));

        // Build block
        Block b = client.generateBlock(wrongSecretManager, new GenerateBlockOptions().withInputs(inputs).withOutput(output));

        // Post the block
        BlockId blockId = client.postBlock(b);

        // Print the block ID.
        System.out.println(blockId);
    }

}
