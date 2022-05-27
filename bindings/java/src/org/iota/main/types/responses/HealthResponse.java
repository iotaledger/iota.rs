package org.iota.main.types.responses;

public class HealthResponse extends ClientResponse {

    private boolean health;

    public HealthResponse(BaseApiResponse response) {
        super(response);
        health = response.getPayload().getAsBoolean();
    }

    public boolean isHealthy() {
        return health;
    }

    @Override
    public String toString() {
        return "GetHealthResponse{" +
                "response=" + response +
                '}';
    }
}
