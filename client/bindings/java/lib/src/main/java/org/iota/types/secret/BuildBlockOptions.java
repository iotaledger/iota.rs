// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;
import org.iota.types.JsonUtils;
import org.iota.types.Output;
import org.iota.types.UtxoInput;
import org.iota.types.ids.BlockId;
import org.iota.types.Burn;

public class BuildBlockOptions {

    private Integer coinType;
    private Integer accountIndex;
    private Integer initialAddressIndex;
    private UtxoInput[] inputs;
    private Range inputRange;
    private ClientBlockBuilderOutputAddress output;
    private ClientBlockBuilderOutputAddress outputHex;
    private Output[] outputs;
    private String customRemainderAddress;
    private String tag;
    private String data;
    private BlockId[] parents;
    private Burn burn;

    public BuildBlockOptions withCoinType(Integer coinType) {
        this.coinType = coinType;
        return this;
    }

    public BuildBlockOptions withAccountIndex(Integer accountIndex) {
        this.accountIndex = accountIndex;
        return this;
    }

    public BuildBlockOptions withInitialAddressIndex(Integer initialAddressIndex) {
        this.initialAddressIndex = initialAddressIndex;
        return this;
    }

    public BuildBlockOptions withInputs(UtxoInput[] inputs) {
        this.inputs = inputs;
        return this;
    }

    public BuildBlockOptions withInputRange(Range inputRange) {
        this.inputRange = inputRange;
        return this;
    }

    public BuildBlockOptions withOutput(ClientBlockBuilderOutputAddress output) {
        this.output = output;
        return this;
    }

    public BuildBlockOptions withOutputHex(ClientBlockBuilderOutputAddress outputHex) {
        this.outputHex = outputHex;
        return this;
    }

    public BuildBlockOptions withOutputs(Output[] outputs) {
        this.outputs = outputs;
        return this;
    }

    public BuildBlockOptions withCustomRemainderAddress(String customRemainderAddress) {
        this.customRemainderAddress = customRemainderAddress;
        return this;
    }

    public BuildBlockOptions withTag(String tag) {
        this.tag = tag;
        return this;
    }

    public BuildBlockOptions withData(String data) {
        this.data = data;
        return this;
    }

    public BuildBlockOptions withParents(BlockId[] parents) {
        this.parents = parents;
        return this;
    }

    public BuildBlockOptions withBurn(Burn burn) {
        this.burn = burn;
        return this;
    }

    public static class ClientBlockBuilderOutputAddress {
        private String address;
        private String amount;

        public ClientBlockBuilderOutputAddress(String address, String amount) {
            this.address = address;
            this.amount = amount;
        }

        public JsonObject getAsJson() {
            JsonObject o = new JsonObject();
            o.addProperty("address", address);
            o.addProperty("amount", amount);

            return o;
        }
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("coinType", coinType);
        o.addProperty("accountIndex", accountIndex);
        o.addProperty("initialAddressIndex", initialAddressIndex);
        o.add("inputs", JsonUtils.toJson(inputs));
        o.add("inputRange", inputRange != null ? inputRange.getAsJson() : null);
        o.add("output", output != null ? output.getAsJson() : null);
        o.add("outputHex", outputHex != null ? outputHex.getAsJson() : null);
        o.add("outputs", JsonUtils.toJson(outputs));
        o.addProperty("customRemainderAddress", customRemainderAddress);
        o.addProperty("tag", tag);
        o.addProperty("data", data);
        o.add("parents", JsonUtils.toJson(parents));
        o.add("burn", burn != null ? burn.toJson() : null);

        return o;
    }

}