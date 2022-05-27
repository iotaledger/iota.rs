package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class HealthResponse extends ClientResponse {

    private boolean health;

    public HealthResponse(BaseApiResponse response) {
        super(response);
        health = response.getPayload().getAsBoolean();
    }

    public boolean isHealthy() {
        return health;
    }

}
