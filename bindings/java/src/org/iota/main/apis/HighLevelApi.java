package org.iota.main.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.iota.main.types.*;
import org.iota.main.types.responses.ClientResponse;
import org.iota.main.types.secret.SecretManager;

import java.util.LinkedHashMap;
import java.util.Map;

public class HighLevelApi extends BaseApi {

    public HighLevelApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public Output[] getOutputs(OutputId[] outputIds) throws ClientException {
        JsonArray a = new JsonArray();
        for (OutputId id : outputIds)
            a.add(id.toString());
        JsonObject o = new JsonObject();
        o.add("outputIds", a);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "GetOutputs", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Output[] outputs = new Output[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            outputs[i] = new Output(responsePayload.get(i).getAsJsonObject());

        return outputs;
    }

    public Output[] tryGetOutputs(OutputId[] outputIds) throws ClientException {
        JsonArray a = new JsonArray();
        for (OutputId id : outputIds)
            a.add(id.toString());
        JsonObject o = new JsonObject();
        o.add("outputIds", a);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "TryGetOutputs", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Output[] outputs = new Output[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            outputs[i] = new Output(responsePayload.get(i).getAsJsonObject());

        return outputs;
    }

    public Block[] findBlocks(BlockId[] blockIds) throws ClientException {
        JsonArray a = new JsonArray();
        for (BlockId id : blockIds)
            a.add(id.toString());
        JsonObject o = new JsonObject();
        o.add("blockIds", a);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindBlocks", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        Block[] blocks = new Block[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            blocks[i] = new Block(responsePayload.get(i).getAsJsonObject());

        return blocks;
    }

    public Map.Entry<BlockId, Block> retry(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Retry", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return Map.entry(new BlockId(responsePayload.get("blockId").getAsString()), new Block(responsePayload.get("block").getAsJsonObject()));
    }

    public LinkedHashMap<BlockId, Block> retryUntilIncluded(BlockId blockId, int interval, int maxAttempts) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());
        o.addProperty("interval", interval);
        o.addProperty("maxAttempts", maxAttempts);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "RetryUntilIncluded", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        LinkedHashMap<BlockId, Block> blocks = new LinkedHashMap<BlockId, Block>();
        for (JsonElement entry : responsePayload) {
            JsonObject e = entry.getAsJsonObject();
            blocks.put(new BlockId(e.get("blockId").getAsString()), new Block(e.get("block").getAsJsonObject()));
        }

        return blocks;
    }

    public String consolidateFunds(SecretManager secretManager, int accountIndex, int addressRange) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.addProperty("accountIndex", accountIndex);
        o.addProperty("addressRange", addressRange);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ConsolidateFunds", o.toString()));
        String responsePayload = response.getPayload().getAsString();

        return responsePayload;
    }

    public OutputId[] findInputs(String[] addresses, int amount) throws ClientException {
        JsonArray a = new JsonArray();
        for (String address : addresses)
            a.add(address);
        JsonObject o = new JsonObject();
        o.add("addresses", a);
        o.addProperty("amount", amount);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindInputs", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());

        return outputIds;
    }

    public OutputId[] findOutputs(OutputId[] outputIds, String[] addresses) throws ClientException {
        JsonArray outputIdsJson = new JsonArray();
        JsonArray addressesJson = new JsonArray();
        for (OutputId outputId : outputIds)
            outputIdsJson.add(outputId.toString());
        for (String address : addresses)
            addressesJson.add(address);

        JsonObject o = new JsonObject();
        o.add("outputIds", outputIdsJson);
        o.add("addresses", addressesJson);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FindOutputs", o.toString()));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        OutputId[] outputIdsRet = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++)
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());

        return outputIdsRet;

    }

    public Map.Entry<BlockId, Block> reattach(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Reattach", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return Map.entry(new BlockId(responsePayload.get("blockId").getAsString()), new Block(responsePayload.get("block").getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> reattachUnchecked(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "ReattachUnchecked", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return Map.entry(new BlockId(responsePayload.get("blockId").getAsString()), new Block(responsePayload.get("block").getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> promote(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "Promote", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return Map.entry(new BlockId(responsePayload.get("blockId").getAsString()), new Block(responsePayload.get("block").getAsJsonObject()));
    }

    public Map.Entry<BlockId, Block> promoteUnchecked(BlockId blockId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("blockId", blockId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "PromoteUnchecked", o.toString()));
        JsonObject responsePayload = response.getPayload().getAsJsonObject();

        return Map.entry(new BlockId(responsePayload.get("blockId").getAsString()), new Block(responsePayload.get("block").getAsJsonObject()));
    }

}
