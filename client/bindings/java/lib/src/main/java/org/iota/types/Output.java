// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class Output extends AbstractObject {

    public Output(JsonObject jsonObject) {
        super(jsonObject);
    }

    public Output(String jsonObject) {
        super(jsonObject);
    }

}