package org.iota.main.apis;

import com.google.gson.GsonBuilder;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.SecretManager;

public class HighLevelApi extends BaseApi {

    public HighLevelApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String getOutputs(String[] outputIds) {
        String outputIdsJson = new GsonBuilder().create().toJson(outputIds);
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputs", "{\"outputIds\":" + outputIdsJson + "}"));
    }

    public String tryGetOutputs(String[] outputIds) {
        String outputIdsJson = new GsonBuilder().create().toJson(outputIds);
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "TryGetOutputs", "{\"outputIds\":" + outputIdsJson + "}"));
    }

    public String findMessages(String[] messageIds) {
        String messageIdsJson = new GsonBuilder().create().toJson(messageIds);
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindMessages", "{\"messageIds\":" + messageIdsJson + "}"));
    }

    public String retry(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Retry", "{\"messageId\":" + messageId + "}"));
    }

    public String retryUntilIncluded(String messageId, int interval, int maxAttempts) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "RetryUntilIncluded", "{\"messageId\": \"" + messageId + "\", \"interval\": " + interval + ", \"maxAttempts\": " + maxAttempts + "}"));
    }

    public String consolidateFunds(SecretManager secretManager, int accountIndex, int addressRange) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ConsolidateFunds", "{\"secretManager\": \"" + secretManager.toString() + "\", \"accountIndex\": " + accountIndex + ", \"addressRange\": " + addressRange + "}"));
    }

    public String findInputs(String[] addresses, int amount) {
        String addressesJson = new GsonBuilder().create().toJson(addresses);
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindInputs", "{\"addresses\": " + addressesJson + ", \"amount\": " + amount + "}"));
    }

    public String findOutputs(String[] outputs, String[] addresses) {
        String outputsJson = new GsonBuilder().create().toJson(outputs);
        String addressesJson = new GsonBuilder().create().toJson(addresses);
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindOutputs", "{\"addresses\": " + addressesJson + ", \"outputs\": " + outputsJson + "}"));
    }

    public String reattach(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Reattach", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String reattachUnchecked(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ReattachUnchecked", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String promote(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Promote", "{\"messageId\":\"" + messageId + "\"}"));
    }

    public String promoteUnchecked(String messageId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PromoteUnchecked", "{\"messageId\":\"" + messageId + "\"}"));
    }

}
