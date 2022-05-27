package org.iota.main.types.responses;

public class Bech32ToHexResponse extends ClientResponse {

    private String hexAddress;

    public Bech32ToHexResponse(BaseApiResponse response) {
        super(response);

        this.hexAddress = response.getPayload().getAsString();
    }

    public String getHexAddress() {
        return hexAddress;
    }

    @Override
    public String toString() {
        return "Bech32ToHexResponse{" +
                "response=" + response +
                '}';
    }
}
