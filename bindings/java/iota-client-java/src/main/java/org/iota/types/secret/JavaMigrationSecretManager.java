// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;
import org.apache.commons.codec.binary.Hex;

/*
An incorrect seed conversion from Java to Rust in February 2022 resulted in incorrectly derived addresses. See https://github.com/iotaledger/iota.rs/pull/800 for more details.
This `JavaMigrationSecretManager` gives access to the funds located on the incorrectly derived addresses.
 */
public class JavaMigrationSecretManager extends SecretManager {

    private String wronglyParsedHexSeed;

    public JavaMigrationSecretManager(String originalHexSeed) {
        wronglyParsedHexSeed = Hex.encodeHexString(originalHexSeed.getBytes());
    }

    @Override
    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("HexSeed", wronglyParsedHexSeed);
        return o;
    }
}