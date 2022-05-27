package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class GenerateAddressesResponse extends ClientResponse {

    private String[] addresses;

    public GenerateAddressesResponse(BaseApiResponse response) {
        super(response);

        JsonArray addresses = response.getPayload().getAsJsonArray();
        this.addresses = new String[addresses.size()];
        for (int i = 0; i < addresses.size(); i++) {
            this.addresses[i] = addresses.get(i).getAsString();
        }
    }

    public String[] getAddresses() {
        return addresses;
    }

    @Override
    public String toString() {
        return "GenerateAddressesResponse{" +
                "response=" + response +
                '}';
    }
}
