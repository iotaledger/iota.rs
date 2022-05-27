package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class PostBlockResponse extends ClientResponse {

    private String blockId;

    public PostBlockResponse(BaseApiResponse response) {
        super(response);
        this.blockId = response.getPayload().getAsString();
    }

    public String getBlockId() {
        return blockId;
    }

}
