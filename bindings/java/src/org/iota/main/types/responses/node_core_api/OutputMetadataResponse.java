package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.OutputMetadata;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class OutputMetadataResponse extends ClientResponse {

    private OutputMetadata outputMetadata;

    public OutputMetadataResponse(BaseApiResponse response) {
        super(response);

        this.outputMetadata = new OutputMetadata(response.getPayload().getAsJsonObject());
    }

    public OutputMetadata getOutputMetadata() {
        return outputMetadata;
    }

}
