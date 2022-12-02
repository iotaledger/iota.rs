package org.iota.apis;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

public class ClientCommand {

    private String methodName;
    private JsonElement methodParams;

    public ClientCommand(String methodName) {
        this.methodName = methodName;
    }

    public ClientCommand(String methodName, JsonElement methodParams) {
        this.methodName = methodName;
        this.methodParams = methodParams;
    }

    public String getMethodName() {
        return methodName;
    }

    @Override
    public String toString() {
        JsonObject message = new JsonObject();
        message.addProperty("name", methodName);
        message.add("data", methodParams);

        return message.toString();
    }
}