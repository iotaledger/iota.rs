// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class MilestonePayload extends BlockPayload {

    public MilestonePayload(JsonObject jsonObject) {
        super(jsonObject);
    }

    public MilestonePayload(String jsonObject) {
        super(jsonObject);
    }

}