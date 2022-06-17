package org.iota.types;

import java.util.Objects;

public class TransactionId {

    private String transactionId;

    public TransactionId(String transactionId) {
        this.transactionId = transactionId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        TransactionId outputId1 = (TransactionId) o;
        return Objects.equals(transactionId, outputId1.transactionId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(transactionId);
    }

    @Override
    public String toString() {
        return transactionId;
    }
}
