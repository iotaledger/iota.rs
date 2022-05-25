package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class GetBlockRawResponse implements ClientResponse {

    private JsonArray blockBytes;

    public GetBlockRawResponse(JsonArray blockBytes) {
        this.blockBytes = blockBytes;
    }

    public JsonArray getBlockBytes() {
        return blockBytes;
    }

    @Override
    public String toString() {
        return blockBytes.toString();
    }
}
