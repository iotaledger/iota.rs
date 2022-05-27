package org.iota.main.types.responses;

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
