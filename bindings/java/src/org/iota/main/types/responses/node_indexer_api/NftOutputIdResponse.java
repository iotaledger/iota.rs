package org.iota.main.types.responses.node_indexer_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class NftOutputIdResponse extends ClientResponse {

    private String nftOutputId;

    public NftOutputIdResponse(BaseApiResponse response) {
        super(response);

        nftOutputId = response.getPayload().getAsString();
    }

    public String getNftOutputId() {
        return nftOutputId;
    }

}
