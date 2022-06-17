package org.iota.types;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonSyntaxException;

public class AbstractObject {

    private JsonObject jsonObject;

    public AbstractObject(JsonObject jsonObject) {
        this.jsonObject = jsonObject;
    }

    public AbstractObject(String jsonObject) throws JsonSyntaxException {
        Gson gson = new Gson();
        JsonElement element = gson.fromJson(jsonObject, JsonElement.class);
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
