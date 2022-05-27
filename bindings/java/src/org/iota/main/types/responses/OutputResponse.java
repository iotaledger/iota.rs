package org.iota.main.types.responses;

import org.iota.main.types.Output;

public class OutputResponse extends ClientResponse {

    private Output output;

    public OutputResponse(BaseApiResponse response) {
        super(response);
        this.output = new Output(response.getPayload().getAsJsonObject());
    }

    public Output getOutput() {
        return output;
    }

    @Override
    public String toString() {
        return "OutputResponse{" +
                "response=" + response +
                '}';
    }
}
