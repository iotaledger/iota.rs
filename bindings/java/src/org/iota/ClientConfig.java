package org.iota;

public class ClientConfig {
    private String json;

    public ClientConfig(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}
