package org.iota.main.types;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

public class Milestone {

    private JsonObject jsonObject;

    public Milestone(JsonObject jsonObject) {
        this.jsonObject = jsonObject;
    }

    public Milestone(String jsonObject) {
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


