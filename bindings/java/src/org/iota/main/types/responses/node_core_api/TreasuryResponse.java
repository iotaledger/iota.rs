package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class TreasuryResponse extends ClientResponse {

    private String milestoneId;
    private String amount;

    public TreasuryResponse(BaseApiResponse response) {
        super(response);

        milestoneId = response.getPayload().getAsJsonObject().get("milestoneId").getAsString();
        amount = response.getPayload().getAsJsonObject().get("amount").getAsString();
    }

    public String getMilestoneId() {
        return milestoneId;
    }

    public String getAmount() {
        return amount;
    }

}
