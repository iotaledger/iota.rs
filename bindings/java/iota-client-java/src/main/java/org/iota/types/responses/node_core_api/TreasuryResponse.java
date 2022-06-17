package org.iota.types.responses.node_core_api;

import com.google.gson.JsonObject;
import org.iota.types.MilestoneId;

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
