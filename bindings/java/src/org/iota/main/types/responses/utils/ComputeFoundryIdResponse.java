package org.iota.main.types.responses.utils;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class ComputeFoundryIdResponse extends ClientResponse {

    private String foundryId;

    public ComputeFoundryIdResponse(BaseApiResponse response) {
        super(response);

        foundryId = response.getPayload().getAsString();
    }

    public String getFoundryId() {
        return foundryId;
    }

}
