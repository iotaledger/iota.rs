package org.iota.types;

import com.google.gson.JsonObject;

public class UtxoInput extends AbstractObject {

    public UtxoInput(JsonObject jsonObject) {
        super(jsonObject);
    }

    public UtxoInput(String jsonObject) {
        super(jsonObject);
    }

}