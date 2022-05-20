package org.iota.main.types;

public class PreparedTransactionData {

    private String json;

    private PreparedTransactionData() {
        ;
    }

    public PreparedTransactionData(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}


