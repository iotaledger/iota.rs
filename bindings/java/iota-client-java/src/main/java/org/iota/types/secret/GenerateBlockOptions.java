// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;
import org.iota.types.JsonUtils;
import org.iota.types.Output;
import org.iota.types.UtxoInput;
import org.iota.types.ids.BlockId;

public class GenerateBlockOptions {

    private Integer coinType;
    private Integer accountIndex;
    private Integer initialAddressIndex;
    private UtxoInput[] inputs;
    private Range inputRange;
    private ClientBlockBuilderOutputAddress output;
    private ClientBlockBuilderOutputAddress outputHex;
    private Output[] outputs;
    private String customRemainderAddress;
    private byte[] tag;
    private byte[] data;
    private BlockId[] parents;
    private Boolean allowBurning;

    public GenerateBlockOptions withCoinType(Integer coinType) {
        this.coinType = coinType;
        return this;
    }

    public GenerateBlockOptions withAccountIndex(Integer accountIndex) {
        this.accountIndex = accountIndex;
        return this;
    }

    public GenerateBlockOptions withInitialAddressIndex(Integer initialAddressIndex) {
        this.initialAddressIndex = initialAddressIndex;
        return this;
    }

    public GenerateBlockOptions withInputs(UtxoInput[] inputs) {
        this.inputs = inputs;
        return this;
    }

    public GenerateBlockOptions withInputRange(Range inputRange) {
        this.inputRange = inputRange;
        return this;
    }

    public GenerateBlockOptions withOutput(ClientBlockBuilderOutputAddress output) {
        this.output = output;
        return this;
    }

    public GenerateBlockOptions withOutputHex(ClientBlockBuilderOutputAddress outputHex) {
        this.outputHex = outputHex;
        return this;
    }

    public GenerateBlockOptions withOutputs(Output[] outputs) {
        this.outputs = outputs;
        return this;
    }

    public GenerateBlockOptions withCustomRemainderAddress(String customRemainderAddress) {
        this.customRemainderAddress = customRemainderAddress;
        return this;
    }

    public GenerateBlockOptions withTag(byte[] tag) {
        this.tag = tag;
        return this;
    }

    public GenerateBlockOptions withData(byte[] data) {
        this.data = data;
        return this;
    }

    public GenerateBlockOptions withParents(BlockId[] parents) {
        this.parents = parents;
        return this;
    }

    public GenerateBlockOptions withAllowBurning(Boolean allowBurning) {
        this.allowBurning = allowBurning;
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
        o.add("tag", JsonUtils.toJson(tag));
        o.add("data", JsonUtils.toJson(data));
        o.add("parents", JsonUtils.toJson(parents));
        o.addProperty("allowBurning", allowBurning);

        return o;
    }

}