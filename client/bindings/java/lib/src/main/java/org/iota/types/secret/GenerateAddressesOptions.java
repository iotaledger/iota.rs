// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;

public class GenerateAddressesOptions {
    private Integer coinType;
    private Integer accountIndex;
    private Range range;
    private Boolean internal;
    private String bech32Hrp;
    private GenerateAddressOptions options;

    public GenerateAddressesOptions withCoinType(Integer coinType) {
        this.coinType = coinType;
        return this;
    }

    public GenerateAddressesOptions withAccountIndex(Integer accountIndex) {
        this.accountIndex = accountIndex;
        return this;
    }

    public GenerateAddressesOptions withRange(Range range) {
        this.range = range;
        return this;
    }

    public GenerateAddressesOptions withInternal(Boolean internal) {
        this.internal = internal;
        return this;
    }

    public GenerateAddressesOptions withBech32Hrp(String bech32Hrp) {
        this.bech32Hrp = bech32Hrp;
        return this;
    }

    public GenerateAddressesOptions withOptions(GenerateAddressOptions options) {
        this.options = options;
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("coinType", coinType);
        o.addProperty("accountIndex", accountIndex);
        o.add("range", range != null ? range.getAsJson() : null);
        o.addProperty("internal", internal);
        o.addProperty("bech32Hrp", bech32Hrp);
        o.add("options", options != null ? options.getAsJson() : null);

        return o;
    }

    static class GenerateAddressOptions {
        private boolean ledgerNanoPrompt;

        public GenerateAddressOptions withLedgerNanoPrompt(boolean ledgerNanoPrompt) {
            this.ledgerNanoPrompt = ledgerNanoPrompt;
            return this;
        }

        public JsonObject getAsJson() {
            JsonObject o = new JsonObject();
            o.addProperty("ledgerNanoPrompt", ledgerNanoPrompt);

            return o;
        }
    }

}