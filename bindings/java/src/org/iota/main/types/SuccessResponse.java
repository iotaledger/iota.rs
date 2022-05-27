package org.iota.main.types;

import com.google.gson.JsonElement;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class SuccessResponse extends ClientResponse {
    private String methodName;
    private JsonElement payload;

    public SuccessResponse(String methodName, JsonElement payload) {
        super(new BaseApiResponse(methodName, payload));
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
