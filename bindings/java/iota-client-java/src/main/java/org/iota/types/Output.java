package org.iota.types;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

public class Output {

    private JsonObject jsonObject;

    public Output(JsonObject jsonObject) {
        this.jsonObject = jsonObject;
    }

    public Output(String jsonObject) {
        Gson gson = new Gson();
        JsonElement element = gson.fromJson(jsonObject, JsonElement.class);
        this.jsonObject = element.getAsJsonObject();
    }

    public JsonObject getAsJsonObject() {
        return jsonObject;
    }

    @Override
    public String toString() {
        return jsonObject.toString();
    }
}




