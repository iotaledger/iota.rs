package org.iota.main.types;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

public class Node {

    private JsonObject jsonObject;

    public Node(JsonObject jsonObject) {
        this.jsonObject = jsonObject;
    }

    public Node(String jsonObject) {
        Gson gson = new Gson();
        JsonElement element = gson.fromJson (jsonObject, JsonElement.class);
        this.jsonObject = element.getAsJsonObject();
    }

    public JsonObject getJson() {
        return jsonObject;
    }

    @Override
    public String toString() {
        return jsonObject.toString();
    }
}


