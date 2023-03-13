import com.google.gson.JsonElement;
import org.iota.Client;
import org.iota.apis.NodeIndexerApi;
import org.iota.types.ClientConfig;
import org.iota.types.NativeToken;
import org.iota.types.Output;
import org.iota.types.OutputMetadata;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.OutputId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class GetAddressBalance {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Generate the addresses from the given mnemonic.
        MnemonicSecretManager secretManager = new MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river");
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)));

        // Get the interesting output ids.
        OutputId[] outputIds = client.getBasicOutputIds(new NodeIndexerApi.QueryParams()
                .withParam("address", addresses[0])
                .withParam("hasExpiration", false)
                .withParam("hasTimelock", false)
                .withParam("hasStorageDepositReturn", false)
        ).getItems();

        // Get the outputs.
        List<Map.Entry<Output, OutputMetadata>> outputs = client.getOutputs(outputIds);

        // Calculate the total amount and native tokens.
        int total = 0;
        Map<String, Integer> nativeTokens = new HashMap();
        for (Map.Entry<Output, OutputMetadata> entry : outputs) {
            Output o = entry.getKey();

            if (o.toJson().has("nativeTokens")) {
                for(JsonElement elem: o.toJson().get("nativeTokens").getAsJsonArray()) {
                    NativeToken nativeToken = new NativeToken(elem.getAsJsonObject());
                    String tokenId = nativeToken.toJson().get("id").getAsString();
                    String amount = nativeToken.toJson().get("amount").getAsString().replace("0x", "");

                    if(nativeTokens.containsKey(tokenId))
                        nativeTokens.put(tokenId, nativeTokens.get(tokenId) + Integer.parseInt(amount, 16));
                    else
                        nativeTokens.put(tokenId, Integer.parseInt(amount, 16));
                }
            }

            total += o.toJson().get("amount").getAsInt();
        }

        System.out.println("total balance: " + total);
        System.out.println("native tokens: " + nativeTokens);
    }
}