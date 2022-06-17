package org.iota.types;

import com.google.gson.JsonObject;

public class BlockPayload extends AbstractObject {

    public BlockPayload(JsonObject jsonObject) {
        super(jsonObject);
    }

    public BlockPayload(String jsonObject) {
        super(jsonObject);
    }

}

