// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;

public class SeedSecretManager extends SecretManager {

    private String hexSeed;

    public SeedSecretManager(String hexSeed) {
        this.hexSeed = hexSeed;
    }

    @Override
    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("hexSeed", hexSeed);

        return o;
    }
}


