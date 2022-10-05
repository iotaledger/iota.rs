// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class NativeToken extends AbstractObject {

    public NativeToken(JsonObject jsonObject) {
        super(jsonObject);
    }

    public NativeToken(String jsonObject) {
        super(jsonObject);
    }

}