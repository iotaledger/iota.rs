// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.OutputId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.Range;
import org.iota.types.secret.SecretManager;

import java.util.AbstractMap.SimpleEntry;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

public class HighLevelApi extends BaseApi {

    public HighLevelApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public List<Map.Entry<Output, OutputMetadata>> getOutputs(OutputId[] outputIds) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("outputIds", JsonUtils.toJson(outputIds));

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("GetOutputs", o));

        List<Map.Entry<Output, OutputMetadata>> outputs = new ArrayList<>();
        for (int i = 0; i < responsePayload.size(); i++) {
            Output output = new Output(responsePayload.get(i).getAsJsonObject().get("output").getAsJsonObject());
            OutputMetadata metadata = new OutputMetadata(responsePayload.get(i).getAsJsonObject().get("metadata").getAsJsonObject());
            outputs.add(new SimpleEntry(output, metadata));
        }

        return outputs;
    }

    public List<Map.Entry<Output, OutputMetadata>> tryGetOutputs(OutputId[] outputIds) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("outputIds", JsonUtils.toJson(outputIds));

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("TryGetOutputs", o));

        List<Map.Entry<Output, OutputMetadata>> outputs = new ArrayList<>();
        for (int i = 0; i < responsePayload.size(); i++) {
            Output output = new Output(responsePayload.get(i).getAsJsonObject().get("output").getAsJsonObject());
            OutputMetadata metadata = new OutputMetadata(responsePayload.get(i).getAsJsonObject().get("metadata").getAsJsonObject());
            outputs.add(new SimpleEntry(output, metadata));
        }

        return outputs;
    }

    public Block[] findBlocks(BlockId[] blockIds) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("blockIds", JsonUtils.toJson(blockIds));

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("FindBlocks", o));

        Block[] blocks = new Block[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            blocks[i] = new Block(responsePayload.get(i).getAsJsonObject());

        return blocks;
    }

    public Map.Entry<BlockId, Block> retry(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("Retry", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public LinkedHashMap<BlockId, Block> retryUntilIncluded(BlockId blockId, int interval, int maxAttempts) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());
        o.addProperty("interval", interval);
        o.addProperty("maxAttempts", maxAttempts);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("RetryUntilIncluded", o));

        LinkedHashMap<BlockId, Block> blocks = new LinkedHashMap<BlockId, Block>();
        for (JsonElement entry : responsePayload) {
            JsonArray e = entry.getAsJsonArray();
            blocks.put(new BlockId(e.get(0).getAsString()), new Block(e.get(1).getAsJsonObject()));
        }

        return blocks;
    }

    public String consolidateFunds(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("generateAddressesOptions", generateAddressesOptions.getJson());

        String responsePayload = callBaseApi(new ClientCommand("ConsolidateFunds", o)).getAsString();

        return responsePayload;
    }

    public UtxoInput[] findInputs(String[] addresses, int amount) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("addresses", JsonUtils.toJson(addresses));
        o.addProperty("amount", amount);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("FindInputs", o));

        UtxoInput[] inputs = new UtxoInput[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            inputs[i] = new UtxoInput(responsePayload.get(i).getAsJsonObject());

        return inputs;
    }

    public List<Map.Entry<Output, OutputMetadata>> findOutputs(OutputId[] outputIds, String[] addresses) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("outputIds", JsonUtils.toJson(outputIds));
        o.add("addresses", JsonUtils.toJson(addresses));

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("FindOutputs", o));

        List<Map.Entry<Output, OutputMetadata>> outputs = new ArrayList<>();
        for (int i = 0; i < responsePayload.size(); i++) {
            Output output = new Output(responsePayload.get(i).getAsJsonObject().get("output").getAsJsonObject());
            OutputMetadata metadata = new OutputMetadata(responsePayload.get(i).getAsJsonObject().get("metadata").getAsJsonObject());
            outputs.add(new SimpleEntry(output, metadata));
        }

        return outputs;
    }

    public Map.Entry<BlockId, Block> reattach(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("Reattach", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> reattachUnchecked(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("ReattachUnchecked", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> promote(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("Promote", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> promoteUnchecked(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("PromoteUnchecked", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

}
