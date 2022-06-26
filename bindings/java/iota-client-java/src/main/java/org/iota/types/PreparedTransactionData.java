// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class PreparedTransactionData extends AbstractObject {

    public PreparedTransactionData(JsonObject jsonObject) {
        super(jsonObject);
    }

    public PreparedTransactionData(String jsonObject) {
        super(jsonObject);
    }

}