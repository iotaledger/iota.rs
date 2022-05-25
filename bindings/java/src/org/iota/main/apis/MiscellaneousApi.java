package org.iota.main.apis;

import org.iota.main.types.*;
import org.iota.main.types.responses.GetBlockResponse;

public class MiscellaneousApi extends BaseApi {

    public MiscellaneousApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public SuccessResponse generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateAddresses", "{\"secretManager\": \"" + secretManager.toString() + "\", \"generateAddressesOptions\": " + generateAddressesOptions + "}"));
    }

    public SuccessResponse generateMessage(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
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

        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GenerateMessage", "{" + methodParams + "}"));
    }


    public SuccessResponse getNode() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNode"));
    }

    public SuccessResponse getNetworkInfo() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNetworkInfo"));
    }

    public SuccessResponse getNetworkId() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetNetworkId"));
    }

    public SuccessResponse getBech32Hrp() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetBech32Hrp"));
    }

    public SuccessResponse getMinPoWScore() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetMinPoWScore"));
    }

    public SuccessResponse getTipsInterval() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetTipsInterval"));
    }

    public SuccessResponse getLocalPoW() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetLocalPoW"));
    }

    public SuccessResponse getFallbackToLocalPoW() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetFallbackToLocalPoW"));
    }

    public SuccessResponse getUnsyncedNodes() throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "UnsyncedNodes"));
    }

    public SuccessResponse prepareTransaction(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
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

        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PrepareTransaction", "{" + methodParams + "}"));
    }

    public SuccessResponse signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "SignTransaction", "{\"secretManager\": \"" + secretManager.toString() + "\", \"preparedTransactionData\": " + preparedTransactionData + "}"));
    }

    public SuccessResponse storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "StoreMnemonic", "{\"secretManager\": \"" + secretManager.toString() + "\", \"mnemonic\": \"" + mnemonic + "\"}"));
    }

    public GetBlockResponse submitBlockPayload(BlockPayload payload) throws ClientException {
        return (GetBlockResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "SubmitPayload", "{\"payload\":" + payload.toString() + "}"));
    }

}

