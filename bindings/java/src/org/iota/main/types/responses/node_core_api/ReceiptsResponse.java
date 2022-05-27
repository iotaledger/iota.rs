package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonArray;
import org.iota.main.types.Receipt;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class ReceiptsResponse extends ClientResponse {

    private Receipt[] receipts;

    public ReceiptsResponse(BaseApiResponse response) {
        super(response);

        JsonArray receipt = response.getPayload().getAsJsonArray();
        this.receipts = new Receipt[receipt.size()];
        for (int i = 0; i < receipt.size(); i++) {
            this.receipts[i] = new Receipt(receipt.get(i).getAsJsonObject());
        }
    }

    public Receipt[] getReceipts() {
        return receipts;
    }

}
