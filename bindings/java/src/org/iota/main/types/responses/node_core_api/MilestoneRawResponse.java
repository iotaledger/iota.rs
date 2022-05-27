package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonArray;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class MilestoneRawResponse extends ClientResponse {

    private byte[] milestoneBytes;

    public MilestoneRawResponse(BaseApiResponse response) {
        super(response);

        JsonArray blockBytes = response.getPayload().getAsJsonArray();
        this.milestoneBytes = new byte[blockBytes.size()];

        for(int i = 0; i < blockBytes.size(); i++) {
            this.milestoneBytes[i] = blockBytes.get(i).getAsByte();
        }
    }

    public byte[] getMilestoneBytes() {
        return milestoneBytes;
    }

}
