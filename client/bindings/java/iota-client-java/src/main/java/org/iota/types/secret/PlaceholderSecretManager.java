// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonElement;
import com.google.gson.JsonPrimitive;

public class PlaceholderSecretManager extends SecretManager {
    @Override
    public JsonElement getJson() {
        return new JsonPrimitive("placeholder");
    }
}


