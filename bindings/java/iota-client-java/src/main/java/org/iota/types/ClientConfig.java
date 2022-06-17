package org.iota.types;

public class ClientConfig {

    private String json;

    private ClientConfig() {
        ;
    }

    public ClientConfig(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}
