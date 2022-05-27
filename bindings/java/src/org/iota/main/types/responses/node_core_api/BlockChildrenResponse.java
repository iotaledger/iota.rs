package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonArray;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

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

}
