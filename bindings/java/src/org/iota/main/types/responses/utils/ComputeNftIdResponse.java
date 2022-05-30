package org.iota.main.types.responses.utils;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class ComputeNftIdResponse extends ClientResponse {

    private String nftId;

    public ComputeNftIdResponse(BaseApiResponse response) {
        super(response);

        nftId = response.getPayload().getAsString();
    }

    public String getNftId() {
        return nftId;
    }

}
