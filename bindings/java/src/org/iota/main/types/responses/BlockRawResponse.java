package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class BlockRawResponse extends ClientResponse {

    private byte[] blockBytes;

    public BlockRawResponse(BaseApiResponse response) {
        super(response);

        JsonArray blockBytes = response.getPayload().getAsJsonArray();
        this.blockBytes = new byte[blockBytes.size()];

        for(int i = 0; i < blockBytes.size(); i++) {
            this.blockBytes[i] = blockBytes.get(i).getAsByte();
        }
    }

    public byte[] getBlockBytes() {
        return blockBytes;
    }

    @Override
    public String toString() {
        return "GetBlockRawResponse{" +
                "response=" + response +
                '}';
    }
}
