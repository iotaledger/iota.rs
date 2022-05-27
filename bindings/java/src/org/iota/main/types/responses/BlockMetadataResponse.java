package org.iota.main.types.responses;

import org.iota.main.types.BlockMetadata;

public class BlockMetadataResponse extends ClientResponse {

    private BlockMetadata blockMetadata;

    public BlockMetadataResponse(BaseApiResponse response) {
        super(response);

        blockMetadata = new BlockMetadata(response.getPayload().getAsJsonObject());
    }

    public BlockMetadata getBlockMetadata() {
        return blockMetadata;
    }

    @Override
    public String toString() {
        return "BlockMetadataResponse{" +
                "response=" + response +
                '}';
    }
}
