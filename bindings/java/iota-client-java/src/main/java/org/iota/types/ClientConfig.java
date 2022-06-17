package org.iota.types;

import com.google.gson.JsonObject;

public class ClientConfig extends AbstractObject {

    public ClientConfig(JsonObject jsonObject) {
        super(jsonObject);
    }

    public ClientConfig(String jsonObject) {
        super(jsonObject);
    }

}
