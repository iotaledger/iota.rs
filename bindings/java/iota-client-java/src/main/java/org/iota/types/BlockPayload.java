// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public abstract class BlockPayload extends AbstractObject {

    public BlockPayload(JsonObject jsonObject) {
        super(jsonObject);
    }

    public BlockPayload(String jsonObject) {
        super(jsonObject);
    }

}