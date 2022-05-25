package org.iota.main.types;

import com.google.gson.JsonElement;
import org.iota.main.types.responses.ClientResponse;

public class SuccessResponse implements ClientResponse {
    private String methodName;
    private JsonElement payload;

    public SuccessResponse(String methodName, JsonElement payload) {
        this.methodName = methodName;
        this.payload = payload;
    }

    public String getMethodName() {
        return methodName;
    }

    public JsonElement getPayload() {
        return payload;
    }

    @Override
    public String toString() {
        return "SuccessResponse{" +
                "methodName='" + methodName + '\'' +
                ", payload='" + payload.toString() + '\'' +
                '}';
    }
}
