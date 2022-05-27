package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonArray;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

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

}
