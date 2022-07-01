// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class TaggedDataPayload extends BlockPayload {

    public TaggedDataPayload(JsonObject jsonObject) {
        super(jsonObject);
    }

    public TaggedDataPayload(String jsonObject) {
        super(jsonObject);
    }

}