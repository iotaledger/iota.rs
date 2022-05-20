package org.iota.main.apis;

import org.iota.main.types.ClientConfig;

public class UtilsApi extends BaseApi {

    public UtilsApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String bech32ToHex(String bech32) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Bech32ToHex", "{\"bech32\":" + bech32 + "}"));
    }

    public String hexToBech32(String hex, String bech32) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "HexToBech32", "{\"hex\":" + hex + ",\"bech32\":\"" + bech32 + "\"}"));
    }

    public String hexPublicKeyToBech32Address(String hex, String bech32) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "HexPublicKeyToBech32Address", "{\"hex\":" + hex + ",\"bech32\":\"" + bech32 + "\"}"));
    }

    public String parseBech32Address(String address) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ParseBech32Address", "{\"address\":\"" + address + "\"}"));
    }

    public String isAddressValid(String address) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "IsAddressValid", "{\"address\":\"" + address + "\"}"));
    }

    public String generateMnemonic() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateMnemonic"));
    }

    public String mnemonicToHexSeed(String mnemonic) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "MnemonicToHexSeed", "{\"mnemonic\":\"" + mnemonic + "\"}"));
    }

    public String messageId(String message) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "MessageId", "{\"message\":\"" + message + "\"}"));
    }
}

