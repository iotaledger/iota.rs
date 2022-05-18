package org.iota.apis;

public class IndexerQueryParams {
    private String json;

    public IndexerQueryParams() {
        this.json = "[]";
    }

    public IndexerQueryParams(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}
