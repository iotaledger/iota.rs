package org.iota.main.types.secret;

import com.google.gson.JsonElement;
import com.google.gson.JsonPrimitive;

public class LedgerNanoSecretManager extends SecretManager {
    @Override
    public JsonElement getJson() {
        return new JsonPrimitive("LedgerNano");
    }
}


