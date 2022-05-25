package org.iota.main.apis;

import com.google.gson.GsonBuilder;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.SuccessResponse;
import org.iota.main.types.SecretManager;

public class HighLevelApi extends BaseApi {

    public HighLevelApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public SuccessResponse getOutputs(String[] outputIds) throws ClientException {
        String outputIdsJson = new GsonBuilder().create().toJson(outputIds);
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputs", "{\"outputIds\":" + outputIdsJson + "}"));
    }

    public SuccessResponse tryGetOutputs(String[] outputIds) throws ClientException {
        String outputIdsJson = new GsonBuilder().create().toJson(outputIds);
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "TryGetOutputs", "{\"outputIds\":" + outputIdsJson + "}"));
    }

    public SuccessResponse findMessages(String[] messageIds) throws ClientException {
        String messageIdsJson = new GsonBuilder().create().toJson(messageIds);
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindMessages", "{\"messageIds\":" + messageIdsJson + "}"));
    }

    public SuccessResponse retry(String messageId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Retry", "{\"messageId\":" + messageId + "}"));
    }

    public SuccessResponse retryUntilIncluded(String messageId, int interval, int maxAttempts) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "RetryUntilIncluded", "{\"messageId\": \"" + messageId + "\", \"interval\": " + interval + ", \"maxAttempts\": " + maxAttempts + "}"));
    }

    public SuccessResponse consolidateFunds(SecretManager secretManager, int accountIndex, int addressRange) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ConsolidateFunds", "{\"secretManager\": \"" + secretManager.toString() + "\", \"accountIndex\": " + accountIndex + ", \"addressRange\": " + addressRange + "}"));
    }

    public SuccessResponse findInputs(String[] addresses, int amount) throws ClientException {
        String addressesJson = new GsonBuilder().create().toJson(addresses);
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindInputs", "{\"addresses\": " + addressesJson + ", \"amount\": " + amount + "}"));
    }

    public SuccessResponse findOutputs(String[] outputs, String[] addresses) throws ClientException {
        String outputsJson = new GsonBuilder().create().toJson(outputs);
        String addressesJson = new GsonBuilder().create().toJson(addresses);
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindOutputs", "{\"addresses\": " + addressesJson + ", \"outputs\": " + outputsJson + "}"));
    }

    public SuccessResponse reattach(String messageId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Reattach", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public SuccessResponse reattachUnchecked(String messageId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ReattachUnchecked", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public SuccessResponse promote(String messageId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Promote", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public SuccessResponse promoteUnchecked(String messageId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PromoteUnchecked", "{\"messageId\":\"" + messageId + "\"}"));
    }

}
