package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class GetBlockChildrenResponse implements ClientResponse {

    private String[] blockChildren;

    public GetBlockChildrenResponse(JsonArray blockChildren) {
        this.blockChildren = new String[blockChildren.size()];
        for(int i = 0; i < blockChildren.size(); i++) {
            this.blockChildren[i] = blockChildren.get(i).getAsString();
        }
    }

    public String[] getBlockChildren() {
        return blockChildren;
    }

}
