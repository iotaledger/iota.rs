package org.iota.main.apis;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.ClientResponse;

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
        ClientResponse response = new Gson().fromJson(callNativeLibrary(clientConfig.toString(), command.toString()), ClientResponse.class);
        System.out.println(response);

        switch (response.getType()) {
            case "Panic":
                throw new RuntimeException(response.toString());
            case "Error":
                throw new ClientException(command.methodName, response.getPayload().getAsJsonObject().toString());

            default:
                return response;
        }
    }

    protected static class ClientCommand {

        private CommandType commandType;
        private String methodName;
        private JsonElement methodParams;


        public ClientCommand(CommandType commandType, String methodName) {
            this.commandType = commandType;
            this.methodName = methodName;
        }

        public ClientCommand(CommandType commandType, String methodName, JsonElement methodParams) {
            this.commandType = commandType;
            this.methodName = methodName;
            this.methodParams = methodParams;
        }

        @Override
        public String toString() {
            JsonObject payload = new JsonObject();
            payload.addProperty("name", methodName);
            if (methodParams != null)
                payload.add("data", methodParams);

            JsonObject outer = new JsonObject();
            outer.addProperty("cmd", commandType.toString());
            outer.add("payload", payload);

            return outer.toString();
        }

        protected enum CommandType {
            CallClientMethod
        }
    }
}