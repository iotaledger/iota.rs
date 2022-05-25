package org.iota.main.apis;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import org.iota.main.types.Block;
import org.iota.main.types.ClientException;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.responses.*;

public class BaseApi {

    protected ClientConfig clientConfig;

    protected BaseApi(ClientConfig clientConfig) {
        this.clientConfig = clientConfig;
    }

    static {
        System.loadLibrary("iota_client");
    }

    private static native String callNativeLibrary(String clientConfig, String clientCommand);

    protected ClientResponse callBaseApi(ClientCommand command) throws ClientException {
        System.out.println(command);
        BaseApiResponse response = new Gson().fromJson(callNativeLibrary(clientConfig.toString(), command.toString()), BaseApiResponse.class);
        System.out.println(response);
        switch (response.type) {
            case "Panic":
                throw new RuntimeException(response.payload.toString());
            case "Error":
                throw new ClientException(command.methodName, response.payload.getAsJsonObject());
                // Node Core API responses
            case "Health": {
                return new GetHealthResponse(response.payload.getAsBoolean());
            }
            case "Info": {
                return new GetNodeInfoResponse(response.payload.getAsJsonObject());
            }
            case "Tips": {
                return new GetTipsResponse(response.payload.getAsJsonArray());
            }
            case "PostBlockSuccessful": {
                return new PostBlockResponse(response.payload.getAsString());
            }
            case "Block":
            case "GeneratedBlock": {
                return new GetBlockResponse(new Block(response.payload.getAsJsonObject()));
            }
            case "BlockRaw": {
                return new GetBlockRawResponse(response.payload.getAsJsonArray());
            }
            case "BlockMetadata": {
                return new GetBlockMetadataResponse(response.payload.getAsJsonObject());
            }
            case "BlockChildren": {
                return new GetBlockChildrenResponse(response.payload.getAsJsonArray());
            }



            default:{
                System.out.println(response.type);
                throw new RuntimeException("no match");
            }
        }
    }

    protected static class ClientCommand {

        private CommandType commandType;
        private String methodName;
        private String methodParams;


        public ClientCommand(CommandType commandType, String methodName) {
            this.commandType = commandType;
            this.methodName = methodName;
        }

        public ClientCommand(CommandType commandType, String methodName, String methodParams) {
            this.commandType = commandType;
            this.methodName = methodName;
            this.methodParams = methodParams;
        }

        @Override
        public String toString() {
            return "{\"cmd\":\"" + commandType.toString() + "\",\"payload\":{\"name\":\"" + methodName + "\"" + (methodParams != null ? ",\"data\":" + methodParams : "") + "}}";
        }

        protected enum CommandType {
            CallClientMethod
        }
    }
}

class BaseApiResponse {
    String type;
    JsonElement payload;

    @Override
    public String toString() {
        return "BaseApiResponse{" +
                "type='" + type + '\'' +
                ", payload=" + payload +
                '}';
    }
}
