package org.iota.main.types.responses;

import com.google.gson.JsonArray;

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

    @Override
    public String toString() {
        return "MilestoneRawResponse{" +
                "response=" + response +
                '}';
    }
}
