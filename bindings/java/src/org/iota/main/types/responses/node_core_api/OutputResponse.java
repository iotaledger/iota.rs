package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.Output;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class OutputResponse extends ClientResponse {

    private Output output;

    public OutputResponse(BaseApiResponse response) {
        super(response);
        this.output = new Output(response.getPayload().getAsJsonObject());
    }

    public Output getOutput() {
        return output;
    }

}
