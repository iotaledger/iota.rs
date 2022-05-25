package org.iota.main.types.responses;

import com.google.gson.JsonArray;

public class GetTipsResponse implements ClientResponse {

    private String[] tips;

    public GetTipsResponse(JsonArray tips) {
        this.tips = new String[tips.size()];
        for(int i = 0; i < tips.size(); i++) {
            this.tips[i] = tips.get(i).getAsString();
        }
    }

    public String[] getTips() {
        return tips;
    }
}
