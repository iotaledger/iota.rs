package org.iota.main.types.responses;

import org.iota.main.types.OutputMetadata;

public class OutputMetadataResponse extends ClientResponse {

    private OutputMetadata outputMetadata;

    public OutputMetadataResponse(BaseApiResponse response) {
        super(response);

        this.outputMetadata = new OutputMetadata(response.getPayload().getAsJsonObject());
    }

    public OutputMetadata getOutputMetadata() {
        return outputMetadata;
    }

    @Override
    public String toString() {
        return "OutputMetadataResponse{" +
                "response=" + response +
                '}';
    }
}
