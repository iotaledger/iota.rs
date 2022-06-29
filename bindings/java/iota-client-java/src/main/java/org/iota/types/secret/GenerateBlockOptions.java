// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.Output;
import org.iota.types.UtxoInput;
import org.iota.types.ids.BlockId;

import java.util.List;

public class GenerateBlockOptions {

    private Integer coinType;
    private Integer accountIndex;
    private Integer initialAddressIndex;
    private List<UtxoInput> inputs;
    private Range inputRange;
    private ClientBlockBuilderOutputAddress output;
    private ClientBlockBuilderOutputAddress outputHex;
    private List<Output> outputs;
    private String customRemainderAddress;
    private byte[] tag;
    private byte[] data;
    private List<BlockId> parents;
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

    public GenerateBlockOptions withInputs(List<UtxoInput> inputs) {
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

    public GenerateBlockOptions withOutputs(List<Output> outputs) {
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

    public GenerateBlockOptions withParents(List<BlockId> parents) {
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

        if(inputs != null) {
            JsonArray array = new JsonArray();
            for(UtxoInput input: inputs)
                array.add(input.getJson());
            o.add("inputs", array);
        } else {
            o.add("inputs", null);
        }

        o.add("inputRange", inputRange != null ? inputRange.getAsJson() : null);
        o.add("output", output != null ? output.getAsJson() : null);
        o.add("outputHex", outputHex != null ? outputHex.getAsJson() : null);

        if(outputs != null) {
            JsonArray array = new JsonArray();
            for(Output output: outputs)
                array.add(output.getJson());
            o.add("outputs", array);
        } else {
            o.add("outputs", null);
        }

        o.addProperty("customRemainderAddress", customRemainderAddress);

        if(tag != null) {
            JsonArray array = new JsonArray();
            for(byte tagByte: tag)
                array.add(tagByte & 0xFF);
            o.add("tag", array);
        } else {
            o.add("tag", null);
        }

        if(data != null) {
            JsonArray array = new JsonArray();
            for(byte dataByte: data)
                array.add(dataByte & 0xFF);
            o.add("data", array);
        } else {
            o.add("data", null);
        }

        if(parents != null) {
            JsonArray array = new JsonArray();
            for(BlockId parent: parents)
                array.add(parent.toString());
            o.add("parents", array);
        } else {
            o.add("parents", null);
        }

        o.addProperty("allowBurning", allowBurning);

        return o;
    }

}