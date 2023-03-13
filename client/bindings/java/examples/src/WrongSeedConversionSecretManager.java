import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;
import org.iota.Client;
import org.iota.apis.NodeIndexerApi;
import org.iota.types.*;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.OutputId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.BuildBlockOptions;
import org.iota.types.secret.Range;
import org.iota.types.secret.SeedSecretManager;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/*
An incorrect seed conversion from Java to Rust in February 2022 resulted in incorrectly derived addresses. See https://github.com/iotaledger/iota.rs/pull/800 for more details.
This example shows how to access and migrate the funds located on the incorrectly derived addresses.
This example will try to migrate funds from the first 50 addresses of the seed.
 */
public class WrongSeedConversionSecretManager {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // The hex seed that is affected by the seed conversion bug.
        String hexSeed = "";
        org.iota.types.secret.WrongSeedConversionSecretManager wrongSecretManager = new org.iota.types.secret.WrongSeedConversionSecretManager(hexSeed);

        // Generate the first 50 affected addresses of account index 0.
        GenerateAddressesOptions addressesOptions = new GenerateAddressesOptions().withAccountIndex(0).withRange(new Range(1, 50));
        String[] affectedAddresses = client.generateAddresses(wrongSecretManager, addressesOptions);

        // Get the affected outputs.
        List<OutputId> affectedOutputIds = new ArrayList<>();
        for (String address : affectedAddresses) {
            OutputId[] outputIds = client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address)).getItems();
            affectedOutputIds.addAll(List.of(outputIds));
        }

        if (affectedOutputIds.size() == 0) {
            throw new RuntimeException("cannot find any outputs to migrate");
        }

        // Prepare the inputs for the transaction.
        List<UtxoInput> inputs = new ArrayList<>();
        int amountToMigrate = 0;
        for (Map.Entry<Output, OutputMetadata> e : client.getOutputs(affectedOutputIds.toArray(new OutputId[affectedOutputIds.size()]))) {
            Output output = e.getKey();
            OutputMetadata metadata = e.getValue();

            JsonObject utxoInputJsonObject = new JsonObject();
            utxoInputJsonObject.addProperty("type", output.toJson().get("type").getAsInt());
            utxoInputJsonObject.addProperty("transactionId", metadata.toJson().get("transactionId").getAsString());
            utxoInputJsonObject.addProperty("transactionOutputIndex", metadata.toJson().get("outputIndex").getAsInt());
            inputs.add(new UtxoInput(utxoInputJsonObject));

            amountToMigrate += output.toJson().get("amount").getAsInt();
        }

        // Build the output for the transaction.
        String receiverAddress = client.generateAddresses(new SeedSecretManager(hexSeed), new GenerateAddressesOptions().withRange(new Range(0, 1)))[0];
        BuildBlockOptions.ClientBlockBuilderOutputAddress output = new BuildBlockOptions.ClientBlockBuilderOutputAddress(receiverAddress, Integer.toString(amountToMigrate));

        // Build block.
        Map.Entry<BlockId, Block> b = client.buildAndPostBlock(wrongSecretManager, new BuildBlockOptions().withInputs(inputs.stream().toArray(UtxoInput[]::new)).withOutput(output));

        // Post the block.
        BlockId blockId = client.postBlock(b.getValue());

        // Print the block ID.
        System.out.println(blockId);
    }
}