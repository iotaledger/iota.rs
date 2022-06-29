// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.OutputId;
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
        JsonArray a = new JsonArray();
        for (OutputId id : outputIds)
            a.add(id.toString());
        JsonObject o = new JsonObject();
        o.add("outputIds", a);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputs", o));

        List<Map.Entry<Output, OutputMetadata>> outputs = new ArrayList<>();
        for (int i = 0; i < responsePayload.size(); i++) {
            Output output = new Output(responsePayload.get(i).getAsJsonObject().get("output").getAsJsonObject());
            OutputMetadata metadata = new OutputMetadata(responsePayload.get(i).getAsJsonObject().get("metadata").getAsJsonObject());
            outputs.add(new SimpleEntry(output, metadata));
        }

        return outputs;
    }

    public List<Map.Entry<Output, OutputMetadata>> tryGetOutputs(OutputId[] outputIds) throws ClientException {
        JsonArray a = new JsonArray();
        for (OutputId id : outputIds)
            a.add(id.toString());
        JsonObject o = new JsonObject();
        o.add("outputIds", a);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "TryGetOutputs", o));

        List<Map.Entry<Output, OutputMetadata>> outputs = new ArrayList<>();
        for (int i = 0; i < responsePayload.size(); i++) {
            Output output = new Output(responsePayload.get(i).getAsJsonObject().get("output").getAsJsonObject());
            OutputMetadata metadata = new OutputMetadata(responsePayload.get(i).getAsJsonObject().get("metadata").getAsJsonObject());
            outputs.add(new SimpleEntry(output, metadata));
        }

        return outputs;
    }

    public Block[] findBlocks(BlockId[] blockIds) throws ClientException {
        JsonArray a = new JsonArray();
        for (BlockId id : blockIds)
            a.add(id.toString());
        JsonObject o = new JsonObject();
        o.add("blockIds", a);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindBlocks", o));

        Block[] blocks = new Block[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            blocks[i] = new Block(responsePayload.get(i).getAsJsonObject());

        return blocks;
    }

    public Map.Entry<BlockId, Block> retry(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Retry", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public LinkedHashMap<BlockId, Block> retryUntilIncluded(BlockId blockId, int interval, int maxAttempts) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());
        o.addProperty("interval", interval);
        o.addProperty("maxAttempts", maxAttempts);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "RetryUntilIncluded", o));

        LinkedHashMap<BlockId, Block> blocks = new LinkedHashMap<BlockId, Block>();
        for (JsonElement entry : responsePayload) {
            JsonArray e = entry.getAsJsonArray();
            blocks.put(new BlockId(e.get(0).getAsString()), new Block(e.get(1).getAsJsonObject()));
        }

        return blocks;
    }

    public String consolidateFunds(SecretManager secretManager, int accountIndex, Range addressRange) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.addProperty("accountIndex", accountIndex);
        o.add("addressRange", addressRange.getAsJson());

        String responsePayload = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ConsolidateFunds", o)).getAsString();

        return responsePayload;
    }

    public UtxoInput[] findInputs(String[] addresses, int amount) throws ClientException {
        JsonArray a = new JsonArray();
        for (String address : addresses)
            a.add(address);
        JsonObject o = new JsonObject();
        o.add("addresses", a);
        o.addProperty("amount", amount);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindInputs", o));

        UtxoInput[] inputs = new UtxoInput[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            inputs[i] = new UtxoInput(responsePayload.get(i).getAsJsonObject());

        return inputs;
    }

    public List<Map.Entry<Output, OutputMetadata>> findOutputs(OutputId[] outputIds, String[] addresses) throws ClientException {
        JsonArray outputIdsJson = new JsonArray();
        JsonArray addressesJson = new JsonArray();
        for (OutputId outputId : outputIds)
            outputIdsJson.add(outputId.toString());
        for (String address : addresses)
            addressesJson.add(address);

        JsonObject o = new JsonObject();
        o.add("outputIds", outputIdsJson);
        o.add("addresses", addressesJson);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindOutputs", o));

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

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Reattach", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> reattachUnchecked(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ReattachUnchecked", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> promote(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Promote", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> promoteUnchecked(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PromoteUnchecked", o));

        return Map.entry(new BlockId(responsePayload.get(0).getAsString()), new Block(responsePayload.get(1).getAsJsonObject()));
    }

}
