package org.iota.types.responses;

import com.google.gson.JsonElement;

public class ClientResponse {

    private String type;
    private JsonElement payload;

    public ClientResponse(String type, JsonElement payload) {
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
        return "ClientResponse{" +
                "type='" + type + '\'' +
                ", payload=" + payload +
                '}';
    }
}
