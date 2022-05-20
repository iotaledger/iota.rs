package org.iota.main.types;

public class Payload {

    private String json;

    private Payload() {
        ;
    }

    public Payload(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}


