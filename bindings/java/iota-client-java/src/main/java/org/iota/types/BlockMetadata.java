// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class BlockMetadata extends AbstractObject {

    public BlockMetadata(JsonObject jsonObject) {
        super(jsonObject);
    }

    public BlockMetadata(String jsonObject) {
        super(jsonObject);
    }

}