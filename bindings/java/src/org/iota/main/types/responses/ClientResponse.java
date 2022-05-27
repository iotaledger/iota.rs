package org.iota.main.types.responses;

public abstract class ClientResponse {
    protected BaseApiResponse response;

    public ClientResponse(BaseApiResponse response) {
        this.response = response;
    }

    @Override
    public String toString() {
        return "ClientResponse{" +
                "response=" + response +
                '}';
    }
}
