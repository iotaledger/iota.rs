package org.iota.main.types.responses;

import org.iota.main.types.Block;

public class GetBlockResponse implements ClientResponse {

    private Block block;

    public GetBlockResponse(Block block) {
        this.block = block;
    }

    public Block getBlock() {
        return block;
    }

    @Override
    public String toString() {
        return block.toString();
    }
}
