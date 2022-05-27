package org.iota.main.types.responses.node_indexer_api;

import com.google.gson.JsonArray;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class OutputIdsResponse extends ClientResponse {

    private String[] outputIds;

    public OutputIdsResponse(BaseApiResponse response) {
        super(response);

        JsonArray outputIds = response.getPayload().getAsJsonArray();
        this.outputIds = new String[outputIds.size()];
        for (int i = 0; i < outputIds.size(); i++) {
            this.outputIds[i] = outputIds.get(i).getAsString();
        }

    }

    public String[] getOutputIds() {
        return outputIds;
    }

}
