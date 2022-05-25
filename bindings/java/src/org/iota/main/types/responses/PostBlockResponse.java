package org.iota.main.types.responses;

public class PostBlockResponse implements ClientResponse {

    private String blockId;

    public PostBlockResponse(String blockId) {
        this.blockId = blockId;
    }

    public String getBlockId() {
        return blockId;
    }

    @Override
    public String toString() {
        return blockId;
    }
}
