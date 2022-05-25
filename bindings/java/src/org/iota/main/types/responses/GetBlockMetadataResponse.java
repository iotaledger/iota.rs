package org.iota.main.types.responses;

import com.google.gson.JsonObject;

public class GetBlockMetadataResponse implements ClientResponse {

    private JsonObject blockMetadata;

    public GetBlockMetadataResponse(JsonObject blockMetadata) {
        this.blockMetadata = blockMetadata;
    }

    public JsonObject getBlockMetadata() {
        return blockMetadata;
    }

    @Override
    public String toString() {
        return blockMetadata.toString();
    }
}
