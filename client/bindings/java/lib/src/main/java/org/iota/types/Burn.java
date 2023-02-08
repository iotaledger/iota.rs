// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class Burn extends AbstractObject {

    public Burn(JsonObject jsonObject) {
        super(jsonObject);
    }

    public Burn(String jsonObject) {
        super(jsonObject);
    }

}