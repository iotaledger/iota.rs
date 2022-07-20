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
        String libraryName = null;

        if (SystemUtils.IS_OS_LINUX)
            libraryName = "libiota_client.so";
        else if (SystemUtils.IS_OS_MAC)
            libraryName = "libiota_client.dylib";
        else if (SystemUtils.IS_OS_WINDOWS)
            libraryName = "iota_client.dll";
        else throw new RuntimeException("OS not supported");

        try {
            NativeUtils.loadLibraryFromJar("/" + libraryName);
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException("cannot load native library");
        }

    }

    private static native String callNativeLibrary(String clientConfig, String clientCommand);

    protected JsonElement callBaseApi(ClientCommand command) throws ClientException {
        String jsonResponse = callNativeLibrary(clientConfig.getJson().toString(), command.toString());
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

        private String methodName;
        private JsonElement methodParams;

        public ClientCommand(String methodName) {
            this.methodName = methodName;
        }

        public ClientCommand(String methodName, JsonElement methodParams) {
            this.methodName = methodName;
            this.methodParams = methodParams;
        }

        @Override
        public String toString() {
            JsonObject message = new JsonObject();
            message.addProperty("name", methodName);
            message.add("data", methodParams);

            return message.toString();
        }
    }
}