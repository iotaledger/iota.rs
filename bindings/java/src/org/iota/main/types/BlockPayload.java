package org.iota.main.types;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

public class BlockPayload {

    private JsonObject jsonObject;

    public BlockPayload(JsonObject jsonObject) {
        this.jsonObject = jsonObject;
    }

    public BlockPayload(String jsonString) {
        Gson gson = new Gson();
        JsonElement element = gson.fromJson (jsonString, JsonElement.class);
        JsonObject jsonObject = element.getAsJsonObject();
        this.jsonObject = jsonObject;
    }

    public JsonObject getAsJsonObject() {
        return jsonObject;
    }

    @Override
    public String toString() {
        return jsonObject.toString();
    }
}


