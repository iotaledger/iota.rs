package org.iota.main.types.responses.node_indexer_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class OutputIdResponse extends ClientResponse {

    private String outputId;

    public OutputIdResponse(BaseApiResponse response) {
        super(response);

        outputId = response.getPayload().getAsString();
    }

    public String getOutputId() {
        return outputId;
    }

}
