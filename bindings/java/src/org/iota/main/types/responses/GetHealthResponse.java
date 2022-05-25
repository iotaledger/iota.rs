package org.iota.main.types.responses;

public class GetHealthResponse implements ClientResponse {

    private boolean health;

    public GetHealthResponse(boolean health) {
        this.health = health;
    }

    public boolean isHealthy() {
        return health;
    }

    @Override
    public String toString() {
        return Boolean.toString(health);
    }
}
