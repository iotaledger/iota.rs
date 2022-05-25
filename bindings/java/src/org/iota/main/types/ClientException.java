package org.iota.main.types;

import com.google.gson.JsonObject;

public class ClientException extends Exception {

    private String methodName;
    private JsonObject payload;

    public ClientException(String methodName, JsonObject payload) {
        super(payload.toString());
        this.methodName = methodName;
    }

    public String getMethodName() {
        return methodName;
    }

    public JsonObject getPayload() {
        return payload;
    }
}
