package org.iota.main.apis;

import org.iota.main.ClientConfig;

public class HighLevelApi extends BaseApi {

    public HighLevelApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String getOutputs(String[] outputIds) {
        String outputIdsString = "[";
        for (int i = 0; i < outputIds.length; i++) {
            outputIdsString += "\"" + outputIds[i] + "\"";
            if (i < outputIds.length - 1) {
                outputIdsString += ",";
            }
        }
        outputIdsString += "]";
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputs", "{\"outputIds\":\"" + outputIdsString + "\"}"));
    }

}
