package org.iota.main.types;

public class GenerateAddressesOptions {

    private String json;

    private GenerateAddressesOptions() {
        ;
    }

    public GenerateAddressesOptions(String json) {
        this.json = json;
    }

    @Override
    public String toString() {
        return json;
    }
}


