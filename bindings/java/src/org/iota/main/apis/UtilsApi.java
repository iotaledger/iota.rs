package org.iota.main.apis;

import com.google.gson.JsonObject;
import org.iota.main.types.BlockPayload;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.SuccessResponse;
import org.iota.main.types.responses.Bech32ToHexResponse;
import org.iota.main.types.responses.FaucetResponse;
import org.iota.main.types.responses.node_core_api.TransactionIdResponse;
import org.iota.main.types.responses.utils.ComputeAliasIdResponse;
import org.iota.main.types.responses.utils.ComputeFoundryIdResponse;
import org.iota.main.types.responses.utils.ComputeNftIdResponse;

public class UtilsApi extends BaseApi {

    public UtilsApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public Bech32ToHexResponse bech32ToHex(String bech32) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("bech32", bech32);
        return (Bech32ToHexResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Bech32ToHex", o.toString()));
    }

    public SuccessResponse hexToBech32(String hex, String bech32) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "HexToBech32", "{\"hex\":" + hex + ",\"bech32\":\"" + bech32 + "\"}"));
    }

    public SuccessResponse hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "HexPublicKeyToBech32Address", "{\"hex\":" + hex + ",\"bech32\":\"" + bech32 + "\"}"));
    }

    public SuccessResponse parseBech32Address(String address) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ParseBech32Address", "{\"address\":\"" + address + "\"}"));
    }

    public SuccessResponse isAddressValid(String address) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "IsAddressValid", "{\"address\":\"" + address + "\"}"));
    }

    public SuccessResponse generateMnemonic() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateMnemonic"));
    }

    public SuccessResponse mnemonicToHexSeed(String mnemonic) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "MnemonicToHexSeed", "{\"mnemonic\":\"" + mnemonic + "\"}"));
    }

    public SuccessResponse getBlockId(String block) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BlockId", "{\"block\":" + block + "}"));
    }

    public TransactionIdResponse getTransactionId(BlockPayload payload) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("payload", payload.getAsJsonObject());
        return (TransactionIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "TransactionId", o.toString()));
    }

    public FaucetResponse requestFundsFromFaucet(String faucetUrl, String address) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("url", faucetUrl);
        o.addProperty("address", address);
        return (FaucetResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Faucet", o.toString()));
    }

    public ComputeAliasIdResponse computeAliasId(String outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId);
        return (ComputeAliasIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ComputeAliasId", o.toString()));
    }

    public ComputeNftIdResponse computeNftId(String outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId);
        return (ComputeNftIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ComputeNftId", o.toString()));
    }

    public ComputeFoundryIdResponse computeFoundryId(String aliasAddress, int serialNumber, int tokenScheme) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasAddress", aliasAddress);
        o.addProperty("serialNumber", serialNumber);
        o.addProperty("tokenSchemeKind", tokenScheme);
        return (ComputeFoundryIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ComputeFoundryId", o.toString()));
    }
}

