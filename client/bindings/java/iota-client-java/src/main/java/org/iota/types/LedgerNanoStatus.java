// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class LedgerNanoStatus extends AbstractObject {

    public LedgerNanoStatus(JsonObject jsonObject) {
        super(jsonObject);
    }

    public LedgerNanoStatus(String jsonObject) {
        super(jsonObject);
    }

}