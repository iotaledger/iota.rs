// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class TransactionPayload extends BlockPayload {

    public TransactionPayload(JsonObject jsonObject) {
        super(jsonObject);
    }

    public TransactionPayload(String jsonObject) {
        super(jsonObject);
    }

}