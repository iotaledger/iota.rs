package org.iota.main.types;

public class SecretManager {

    private String json;

    private SecretManager() {
        ;
    }

    public SecretManager(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}


