package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.MilestonePayload;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class MilestoneResponse extends ClientResponse {

    private MilestonePayload milestone;

    public MilestoneResponse(BaseApiResponse response) {
        super(response);

        this.milestone = new MilestonePayload(response.getPayload().getAsJsonObject());
    }

    public MilestonePayload getMilestone() {
        return milestone;
    }

}
