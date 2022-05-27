package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.BlockMetadata;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class BlockMetadataResponse extends ClientResponse {

    private BlockMetadata blockMetadata;

    public BlockMetadataResponse(BaseApiResponse response) {
        super(response);

        blockMetadata = new BlockMetadata(response.getPayload().getAsJsonObject());
    }

    public BlockMetadata getBlockMetadata() {
        return blockMetadata;
    }

}
