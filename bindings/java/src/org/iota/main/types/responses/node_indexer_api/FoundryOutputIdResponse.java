package org.iota.main.types.responses.node_indexer_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class FoundryOutputIdResponse extends ClientResponse {

    private String foundryOutputId;

    public FoundryOutputIdResponse(BaseApiResponse response) {
        super(response);

        foundryOutputId = response.getPayload().getAsString();
    }

    public String getFoundryOutputId() {
        return foundryOutputId;
    }

}
