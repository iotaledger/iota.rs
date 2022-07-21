// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonSyntaxException;

import java.util.Objects;

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

    public JsonObject toJson() {
        return jsonObject;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        AbstractObject other = (AbstractObject) o;
        return Objects.equals(this.jsonObject, other.jsonObject);
    }

    @Override
    public int hashCode() {
        return Objects.hash(jsonObject);
    }

    @Override
    public String toString() {
        return jsonObject.toString();
    }

}