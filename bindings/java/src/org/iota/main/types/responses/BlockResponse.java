package org.iota.main.types.responses;

import org.iota.main.types.Block;

public class BlockResponse extends ClientResponse {

    private Block block;

    public BlockResponse(BaseApiResponse response) {
        super(response);
        this.block = new Block(response.getPayload().getAsJsonObject());
    }

    public Block getBlock() {
        return block;
    }

    @Override
    public String toString() {
        return "GetBlockResponse{" +
                "response=" + response +
                '}';
    }
}
