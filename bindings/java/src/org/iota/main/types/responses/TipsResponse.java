package org.iota.main.types.responses;

import com.google.gson.JsonArray;

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

    @Override
    public String toString() {
        return "GetTipsResponse{" +
                "response=" + response +
                '}';
    }
}
