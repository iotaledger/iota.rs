// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class Feature extends AbstractObject {

    public Feature(JsonObject jsonObject) {
        super(jsonObject);
    }

    public Feature(String jsonObject) {
        super(jsonObject);
    }

}