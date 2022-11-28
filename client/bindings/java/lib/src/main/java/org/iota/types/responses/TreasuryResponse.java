// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.responses;

import com.google.gson.JsonObject;
import org.iota.types.ids.MilestoneId;

public class TreasuryResponse {

    private MilestoneId milestoneId;
    private int amount;

    public TreasuryResponse(JsonObject response) {
        milestoneId = new MilestoneId(response.get("milestoneId").getAsString());
        amount = response.get("amount").getAsInt();
    }

    public MilestoneId getMilestoneId() {
        return milestoneId;
    }

    public int getAmount() {
        return amount;
    }

}
