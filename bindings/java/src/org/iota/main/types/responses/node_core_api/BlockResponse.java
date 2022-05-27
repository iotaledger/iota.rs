package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.Block;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class BlockResponse extends ClientResponse {

    private Block block;

    public BlockResponse(BaseApiResponse response) {
        super(response);
        this.block = new Block(response.getPayload().getAsJsonObject());
    }

    public Block getBlock() {
        return block;
    }

}
