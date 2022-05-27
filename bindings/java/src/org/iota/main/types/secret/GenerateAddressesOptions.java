package org.iota.main.types.secret;

import com.google.gson.JsonObject;

public class GenerateAddressesOptions {

    private Integer coinType;
    private Integer accountIndex;
    private Range range;
    private Boolean internal;
    private String bech32Hrp;
    private GenerateAddressMetadata metadata;

    public GenerateAddressesOptions withCoinType(int coinType) {
        this.coinType = coinType;
        return this;
    }

    public GenerateAddressesOptions withAccountIndex(int accountIndex) {
        this.accountIndex = accountIndex;
        return this;
    }

    public GenerateAddressesOptions withInternal(boolean internal) {
        this.internal = internal;
        return this;
    }

    public GenerateAddressesOptions withRange(int start, int end) {
        range = new Range(start, end);
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("coinType", coinType);
        o.addProperty("accountIndex", accountIndex);
        o.add("range", range != null ? range.getAsJson() : null);
        o.addProperty("internal", internal);
        o.addProperty("bech32Hrp", bech32Hrp);
        o.add("metadata", metadata != null ? metadata.getAsJson() : null);
        return o;
    }

    static class GenerateAddressMetadata {
        private boolean synching;

        public GenerateAddressMetadata withSynching(boolean synching) {
            this.synching = synching;
            return this;
        }

        public JsonObject getAsJson() {
            JsonObject o = new JsonObject();
            o.addProperty("synching", synching);
            return o;
        }
    }

}