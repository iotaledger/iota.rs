package org.iota.main.apis;

import org.iota.main.types.*;

public class MiscellaneousApi extends BaseApi {

    public MiscellaneousApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateAddresses", "{\"secretManager\": \"" + secretManager.toString() + "\", \"generateAddressesOptions\": " + generateAddressesOptions + "}"));
    }

    public String generateMessage(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) {
        String methodParams = "{";
        if (secretManager != null) {
            methodParams += "\"secretManager\": \"" + secretManager.toString();
        }
        if (generateAddressesOptions != null) {
            if(secretManager != null) {
                methodParams+= ",";
            }
            methodParams += "\"generateAddressesOptions\": \"" + generateAddressesOptions.toString();
        }
        methodParams += "}";

        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateMessage", "{" + methodParams + "}"));
    }


    public String getNode() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNode"));
    }

    public String getNetworkInfo() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNetworkInfo"));
    }

    public String getNetworkId() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNetworkId"));
    }

    public String getBech32Hrp() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBech32Hrp"));
    }

    public String getMinPoWScore() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMinPoWScore"));
    }

    public String getTipsInterval() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTipsInterval"));
    }

    public String getLocalPoW() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetLocalPoW"));
    }

    public String getFallbackToLocalPoW() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetFallbackToLocalPoW"));
    }

    public String getUnsyncedNodes() {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "UnsyncedNodes"));
    }

    public String prepareTransaction(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) {
        String methodParams = "{";
        if (secretManager != null) {
            methodParams += "\"secretManager\": \"" + secretManager.toString();
        }
        if (generateAddressesOptions != null) {
            if(secretManager != null) {
                methodParams+= ",";
            }
            methodParams += "\"generateAddressesOptions\": \"" + generateAddressesOptions.toString();
        }
        methodParams += "}";

        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PrepareTransaction", "{" + methodParams + "}"));
    }

    public String signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "SignTransaction", "{\"secretManager\": \"" + secretManager.toString() + "\", \"preparedTransactionData\": " + preparedTransactionData + "}"));
    }

    public String storeMnemonic(SecretManager secretManager, String mnemonic) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "StoreMnemonic", "{\"secretManager\": \"" + secretManager.toString() + "\", \"mnemonic\": \"" + mnemonic + "\"}"));
    }

    public String submitPayload(Payload payload) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "SubmitPayload", payload.toString()));
    }

}

