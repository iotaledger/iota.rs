// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;

public class LedgerNanoSecretManager extends SecretManager {
    private boolean isSimulator;

    public LedgerNanoSecretManager(boolean isSimulator) {
        this.isSimulator = isSimulator;
    }

    @Override
    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("ledgerNano", isSimulator);

        return o;
    }
}


