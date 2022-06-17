package org.iota.types;

import com.google.gson.JsonObject;

public class BlockMetadata extends AbstractObject {

    public BlockMetadata(JsonObject jsonObject) {
        super(jsonObject);
    }

    public BlockMetadata(String jsonObject) {
        super(jsonObject);
    }

}