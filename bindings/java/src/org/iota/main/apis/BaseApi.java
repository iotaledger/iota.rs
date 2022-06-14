package org.iota.main.apis;

import com.google.gson.Gson;
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