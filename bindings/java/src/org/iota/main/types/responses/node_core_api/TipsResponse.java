package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonArray;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class TipsResponse extends ClientResponse {

    private String[] tips;

    public TipsResponse(BaseApiResponse response) {
        super(response);

        JsonArray tips = response.getPayload().getAsJsonArray();
        this.tips = new String[tips.size()];
        for(int i = 0; i < tips.size(); i++) {
            this.tips[i] = tips.get(i).getAsString();
        }
    }

    public String[] getTips() {
        return tips;
    }

}
