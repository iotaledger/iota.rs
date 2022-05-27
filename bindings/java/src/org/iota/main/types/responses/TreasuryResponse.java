package org.iota.main.types.responses;

import com.google.gson.JsonArray;
import org.iota.main.types.Receipt;

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

    @Override
    public String toString() {
        return "TreasuryResponse{" +
                "response=" + response +
                '}';
    }
}
