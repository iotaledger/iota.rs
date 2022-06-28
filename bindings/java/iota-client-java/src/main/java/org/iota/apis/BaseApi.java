// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.apache.commons.lang3.SystemUtils;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;

import java.io.IOException;

public class BaseApi {

    protected ClientConfig clientConfig;

    protected BaseApi(ClientConfig clientConfig) {
        this.clientConfig = clientConfig;
    }

    static {
        String libraryPath = null;

        if (SystemUtils.IS_OS_LINUX)
            libraryPath = "/targets/linux-x86-64/iota_client.so";
        else if (SystemUtils.IS_OS_MAC)
            libraryPath = "/targets/mac-x86-64/iota_client.dylib";
        else if (SystemUtils.IS_OS_WINDOWS)
            libraryPath = "/targets/windows-x86-64/iota_client.dll";
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
        String jsonResponse = callNativeLibrary(clientConfig.toString(), command.toString());
        ClientResponse response = new Gson().fromJson(jsonResponse, ClientResponse.class);

        switch (response.type) {
            case "Panic":
                throw new RuntimeException(response.toString());
            case "Error":
                throw new ClientException(command.methodName, response.payload.getAsJsonObject().toString());

            default:
                return response.payload;
        }
    }

    private class ClientResponse {
        String type;
        JsonElement payload;
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