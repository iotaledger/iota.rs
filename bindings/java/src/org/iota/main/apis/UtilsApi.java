package org.iota.main.apis;

import com.google.gson.JsonObject;
import org.iota.main.types.*;
import org.iota.main.types.responses.ClientResponse;

public class UtilsApi extends BaseApi {

    public UtilsApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String bech32ToHex(String bech32) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("bech32", bech32);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Bech32ToHex", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public String hexToBech32(String hex, String bech32) throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "HexToBech32", "{\"hex\":" + hex + ",\"bech32\":\"" + bech32 + "\"}"));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public String hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "HexPublicKeyToBech32Address", "{\"hex\":" + hex + ",\"bech32\":\"" + bech32 + "\"}"));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public String parseBech32Address(String address) throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ParseBech32Address", "{\"address\":\"" + address + "\"}"));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public boolean isAddressValid(String address) throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "IsAddressValid", "{\"address\":\"" + address + "\"}"));
        Boolean responsePayload = response.getPayload().getAsBoolean();

        return responsePayload;
    }

    public String generateMnemonic() throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateMnemonic"));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public String mnemonicToHexSeed(String mnemonic) throws ClientException {
        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "MnemonicToHexSeed", "{\"mnemonic\":\"" + mnemonic + "\"}"));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public BlockId computeBlockId(Block block) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("block", block.getJson());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BlockId", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return new BlockId(responsePayload);
    }

    public TransactionId getTransactionId(TransactionPayload payload) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("payload", payload.getAsJsonObject());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "TransactionId", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return new TransactionId(responsePayload);
    }

    public String requestFundsFromFaucet(String faucetUrl, String address) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("url", faucetUrl);
        o.addProperty("address", address);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Faucet", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public AliasId computeAliasId(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ComputeAliasId", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return new AliasId(responsePayload);
    }

    public NftId computeNftId(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ComputeNftId", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return new NftId(responsePayload);
    }

    public FoundryId computeFoundryId(String aliasAddress, int serialNumber, int tokenScheme) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasAddress", aliasAddress);
        o.addProperty("serialNumber", serialNumber);
        o.addProperty("tokenSchemeKind", tokenScheme);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ComputeFoundryId", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return new FoundryId(responsePayload);
    }
}

