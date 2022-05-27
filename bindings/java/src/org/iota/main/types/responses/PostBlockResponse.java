package org.iota.main.types.responses;

public class PostBlockResponse extends ClientResponse {

    private String blockId;

    public PostBlockResponse(BaseApiResponse response) {
        super(response);
        this.blockId = response.getPayload().getAsString();
    }

    public String getBlockId() {
        return blockId;
    }

    @Override
    public String toString() {
        return "PostBlockResponse{" +
                "response=" + response +
                '}';
    }
}
