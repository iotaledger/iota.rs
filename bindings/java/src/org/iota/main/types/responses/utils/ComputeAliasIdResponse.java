package org.iota.main.types.responses.utils;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class ComputeAliasIdResponse extends ClientResponse {

    private String aliasId;

    public ComputeAliasIdResponse(BaseApiResponse response) {
        super(response);

        aliasId = response.getPayload().getAsString();
    }

    public String getAliasId() {
        return aliasId;
    }

}
