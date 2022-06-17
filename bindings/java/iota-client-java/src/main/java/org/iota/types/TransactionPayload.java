package org.iota.types;

import com.google.gson.JsonObject;

public class TransactionPayload extends AbstractObject {

    public TransactionPayload(JsonObject jsonObject) {
        super(jsonObject);
    }

    public TransactionPayload(String jsonObject) {
        super(jsonObject);
    }

}