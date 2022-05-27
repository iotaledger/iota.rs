package org.iota.main.types.responses.node_core_api;

import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

public class TransactionIdResponse extends ClientResponse {

    private String transactionId;

    public TransactionIdResponse(BaseApiResponse response) {
        super(response);

        this.transactionId = response.getPayload().getAsString();
    }

    public String getTransactionId() {
        return transactionId;
    }

}
