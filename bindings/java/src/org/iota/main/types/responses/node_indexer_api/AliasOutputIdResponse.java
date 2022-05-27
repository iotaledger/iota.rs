package org.iota.main.types.responses.node_indexer_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class AliasOutputIdResponse extends ClientResponse {

    private String aliasOutputId;

    public AliasOutputIdResponse(BaseApiResponse response) {
        super(response);

        aliasOutputId = response.getPayload().getAsString();
    }

    public String getAliasOutputId() {
        return aliasOutputId;
    }

}
