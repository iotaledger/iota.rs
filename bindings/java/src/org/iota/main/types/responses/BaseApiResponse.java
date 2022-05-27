package org.iota.main.types.responses;

import com.google.gson.JsonElement;

public class BaseApiResponse {

    private String type;
    private JsonElement payload;

    public BaseApiResponse(String type, JsonElement payload) {
        this.type = type;
        this.payload = payload;
    }

    public String getType() {
        return type;
    }

    public JsonElement getPayload() {
        return payload;
    }

    @Override
    public String toString() {
        return "BaseApiResponse{" +
                "type='" + type + '\'' +
                ", payload=" + payload +
                '}';
    }
}
