package org.iota.main.types.responses;

import org.iota.main.types.Block;

public class IncludedBlockResponse extends ClientResponse {

    private Block block;

    public IncludedBlockResponse(BaseApiResponse response) {
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
