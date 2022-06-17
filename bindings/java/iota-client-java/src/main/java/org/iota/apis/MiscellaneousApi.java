package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.responses.ClientResponse;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.GenerateBlockOptions;
import org.iota.types.secret.SecretManager;

public class MiscellaneousApi extends BaseApi {

    public MiscellaneousApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String[] generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("options", generateAddressesOptions.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateAddresses", o));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        String[] addresses = new String[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            addresses[i] = responsePayload.get(i).getAsString();
        }

        return addresses;
    }

    public Block generateBlock(SecretManager secretManager, GenerateBlockOptions options) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("options", options.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateBlock", o));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Block(responsePayload);
    }


    public Node getNode() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNode"));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Node(responsePayload);
    }

    public JsonObject getNetworkInfo() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNetworkInfo"));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return responsePayload;
    }

    public int getNetworkId() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNetworkId"));
        Integer responsePayload = response.getPayload().getAsInt();

        return responsePayload;
    }

    public String getBech32Hrp() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBech32Hrp"));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public float getMinPoWScore() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMinPoWScore"));
        Float responsePayload = response.getPayload().getAsFloat();

        return responsePayload;
    }

    public int getTipsInterval() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTipsInterval"));
        Integer responsePayload = response.getPayload().getAsInt();

        return responsePayload;
    }

    public boolean isLocalPow() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetLocalPoW"));
        Boolean responsePayload = response.getPayload().getAsBoolean();

        return responsePayload;
    }

    public boolean isFallbackToLocalPoW() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetFallbackToLocalPoW"));
        Boolean responsePayload = response.getPayload().getAsBoolean();

        return responsePayload;
    }

    public Node[] getUnsyncedNodes() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "UnsyncedNodes"));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Node[] nodes = new Node[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            nodes[i] = new Node(responsePayload.get(i).getAsJsonObject());
        }
        return nodes;
    }

    public PreparedTransactionData prepareTransaction(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("generateAddressesOptions", generateAddressesOptions.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PrepareTransaction", o));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new PreparedTransactionData(responsePayload);
    }

    public BlockPayload signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("preparedTransactionData", preparedTransactionData.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "SignTransaction", o));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new BlockPayload(responsePayload);
    }

    public void storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.addProperty("mnemonic", mnemonic);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "StoreMnemonic", o));
    }

    public Block postBlockPayload(TransactionPayload payload) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("payload", payload.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "SubmitPayload", o));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return new Block(responsePayload);
    }

}

