package org.iota.apis;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.apache.commons.lang3.SystemUtils;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.responses.ClientResponse;

import java.io.IOException;

public class BaseApi {

    protected ClientConfig clientConfig;

    protected BaseApi(ClientConfig clientConfig) {
        this.clientConfig = clientConfig;
    }

    static {
        String libraryPath = null;

        if (SystemUtils.IS_OS_LINUX)
            libraryPath = "/targets/linux/iota_client.so";
        else if (SystemUtils.IS_OS_MAC)
            libraryPath = "/targets/mac/iota_client.dylib";
        else if (SystemUtils.IS_OS_WINDOWS)
            libraryPath = "/targets/windows/iota_client.dll";
        else throw new RuntimeException("OS not supported");

        try {
            NativeUtils.loadLibraryFromJar(libraryPath);
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException("cannot load native library");
        }

    }

    private static native String callNativeLibrary(String clientConfig, String clientCommand);

    protected JsonElement callBaseApi(ClientCommand command) throws ClientException {
        System.out.println(command);
        ClientResponse response = new Gson().fromJson(callNativeLibrary(clientConfig.toString(), command.toString()), ClientResponse.class);
        System.out.println(response);

        switch (response.getType()) {
            case "Panic":
                throw new RuntimeException(response.toString());
            case "Error":
                throw new ClientException(command.methodName, response.getPayload().getAsJsonObject().toString());

            default:
                return response.getPayload();
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