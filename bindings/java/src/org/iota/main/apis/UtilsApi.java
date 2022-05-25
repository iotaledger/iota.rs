package org.iota.main.apis;

import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.SuccessResponse;

public class UtilsApi extends BaseApi {

    public UtilsApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public SuccessResponse bech32ToHex(String bech32) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Bech32ToHex", "{\"bech32\":" + bech32 + "}"));
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
}

