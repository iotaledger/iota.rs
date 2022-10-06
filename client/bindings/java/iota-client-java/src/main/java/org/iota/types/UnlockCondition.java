// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class UnlockCondition extends AbstractObject {

    public UnlockCondition(JsonObject jsonObject) {
        super(jsonObject);
    }

    public UnlockCondition(String jsonObject) {
        super(jsonObject);
    }

}