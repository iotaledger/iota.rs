package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class UtxoChangesResponse extends ClientResponse {

    private int index;
    private String[] consumedOutputs;
    private String[] createdOutputs;

    public UtxoChangesResponse(BaseApiResponse response) {
        super(response);

        index = response.getPayload().getAsJsonObject().get("index").getAsInt();

        JsonArray consumedOutputs = response.getPayload().getAsJsonObject().getAsJsonArray("consumedOutputs");
        this.consumedOutputs = new String[consumedOutputs.size()];
        for (int i = 0; i < consumedOutputs.size(); i++) {
            this.consumedOutputs[i] = consumedOutputs.get(i).getAsString();
        }

        JsonArray createdOutputs = response.getPayload().getAsJsonObject().getAsJsonArray("createdOutputs");
        this.createdOutputs = new String[createdOutputs.size()];
        for (int i = 0; i < createdOutputs.size(); i++) {
            this.createdOutputs[i] = createdOutputs.get(i).getAsString();
        }
    }

    public int getIndex() {
        return index;
    }

    public String[] getConsumedOutputs() {
        return consumedOutputs;
    }

    public String[] getCreatedOutputs() {
        return createdOutputs;
    }

    @Override
    public String toString() {
        return "UtxoChangesResponse{" +
                "response=" + response +
                '}';
    }
}
