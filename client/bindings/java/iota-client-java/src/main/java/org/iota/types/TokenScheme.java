// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class TokenScheme extends AbstractObject {

    public TokenScheme(JsonObject jsonObject) {
        super(jsonObject);
    }

    public TokenScheme(String jsonObject) {
        super(jsonObject);
    }

}