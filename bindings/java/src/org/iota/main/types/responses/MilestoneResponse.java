package org.iota.main.types.responses;

import org.iota.main.types.MilestonePayload;

public class MilestoneResponse extends ClientResponse {

    private MilestonePayload milestone;

    public MilestoneResponse(BaseApiResponse response) {
        super(response);

        this.milestone = new MilestonePayload(response.getPayload().getAsJsonObject());
    }

    public MilestonePayload getMilestone() {
        return milestone;
    }

    @Override
    public String toString() {
        return "MilestoneResponse{" +
                "response=" + response +
                '}';
    }
}
