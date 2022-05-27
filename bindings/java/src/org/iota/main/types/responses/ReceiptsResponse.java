package org.iota.main.types.responses;

import com.google.gson.JsonArray;
import org.iota.main.types.Receipt;

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

    @Override
    public String toString() {
        return "ReceiptsResponse{" +
                "response=" + response +
                '}';
    }
}
