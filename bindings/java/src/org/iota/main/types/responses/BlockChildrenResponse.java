package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class BlockChildrenResponse extends ClientResponse {

    private String[] blockChildren;

    public BlockChildrenResponse(BaseApiResponse response) {
        super(response);

        JsonArray blockChildren = response.getPayload().getAsJsonArray();
        this.blockChildren = new String[blockChildren.size()];
        for (int i = 0; i < blockChildren.size(); i++) {
            this.blockChildren[i] = blockChildren.get(i).getAsString();
        }
    }

    public String[] getBlockChildren() {
        return blockChildren;
    }

    @Override
    public String toString() {
        return "BlockChildrenResponse{" +
                "response=" + response +
                '}';
    }
}
