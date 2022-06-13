package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.Output;
import org.iota.main.types.OutputMetadata;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class OutputResponse extends ClientResponse {

    private Output output;
    private OutputMetadata metadata;

    public OutputResponse(BaseApiResponse response) {
        super(response);
        this.output = new Output(response.getPayload().getAsJsonObject().get("output").getAsJsonObject());
        this.metadata = new OutputMetadata(response.getPayload().getAsJsonObject().get("metadata").getAsJsonObject());
    }

    public OutputMetadata getOutputMetadata() {
        return metadata;
    }

    public Output getOutput() {
        return output;
    }

}
