// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import org.apache.commons.lang3.SystemUtils;
import org.iota.types.ClientConfig;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;

public abstract class NativeApi {

    protected NativeApi(ClientConfig clientConfig) throws InitializeClientException {
        try {
            createMessageHandler(new Gson().toJsonTree(clientConfig).toString());
        } catch (Exception e) {
            throw new InitializeClientException(e.getMessage());
        }
    }

    static {

        Throwable loadFromJavaPathThrowable = null;
        Throwable loadFromJarThrowable = null;

        try {
            loadFromJavaPath();
        } catch (Throwable t) {
            loadFromJavaPathThrowable = t;
        }

        if (loadFromJavaPathThrowable != null) {
            try {
                loadFromJar();
            } catch (Throwable t) {
                loadFromJarThrowable = t;
            }
        }

        if (loadFromJavaPathThrowable != null && loadFromJarThrowable != null) {
            loadFromJavaPathThrowable.printStackTrace();
            loadFromJarThrowable.printStackTrace();
            throw new RuntimeException("cannot load native library");
        }

    }

    private static void loadFromJavaPath() {
        System.loadLibrary("iota_client");
    }

    private static void loadFromJar() throws Throwable {
        String libraryName;

        if (SystemUtils.IS_OS_LINUX)
            libraryName = "libiota_client.so";
        else if (SystemUtils.IS_OS_MAC)
            libraryName = "libiota_client.dylib";
        else if (SystemUtils.IS_OS_WINDOWS)
            libraryName = "iota_client.dll";
        else
            throw new RuntimeException("OS not supported");

        NativeUtils.loadLibraryFromJar("/" + libraryName);
    }

    private static native void createMessageHandler(String config) throws Exception;
    private static native String sendCommand(String clientCommand);

    protected native void destroyHandle();

    protected JsonElement sendCommand(ClientCommand command) throws ClientException {
        String jsonResponse = sendCommand(command.toString());
        ClientResponse response = new Gson().fromJson(jsonResponse, ClientResponse.class);

        switch (response.type) {
            case "panic":
                throw new RuntimeException(response.toString());
            case "error":
                throw new ClientException(command.getMethodName(), response.payload.getAsJsonObject().toString());

            default:
                return response.payload;
        }
    }

    private class ClientResponse {
        String type;
        JsonElement payload;
    }

}