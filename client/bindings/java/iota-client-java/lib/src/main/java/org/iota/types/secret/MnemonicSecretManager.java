// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;

public class MnemonicSecretManager extends SecretManager {
    private String mnemonic;

    public MnemonicSecretManager(String mnemonic) {
        this.mnemonic = mnemonic;
    }

    @Override
    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("mnemonic", mnemonic);

        return o;
    }
}


