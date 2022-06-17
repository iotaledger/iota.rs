package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.ids.AliasId;
import org.iota.types.ids.FoundryId;
import org.iota.types.ids.NftId;
import org.iota.types.ids.OutputId;


public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public OutputId[] getBasicOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BasicOutputIds", o));

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId[] getAliasOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputIds", o));

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId[] getNftOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputIds", o));

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId[] getFoundryOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputIds", o));

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId getAliasOutputIdByAliasId(AliasId aliasId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasId", aliasId.toString());

        String responsePayload = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputId", o)).getAsString();

        return new OutputId(responsePayload);
    }

    public OutputId getNftOutputIdByNftId(NftId nftId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("nftId", nftId.toString());

        String responsePayload = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputId", o)).getAsString();

        return new OutputId(responsePayload);
    }

    public OutputId getFoundryOutputIdByFoundryId(FoundryId foundryId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("foundryId", foundryId.toString());

        String responsePayload = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputId", o)).getAsString();

        return new OutputId(responsePayload);
    }


    public static class QueryParams {

        private JsonArray queryParams = new JsonArray();

        public QueryParams withParam(String name, String value) {
            JsonObject o = new JsonObject();
            o.addProperty(name, value);
            queryParams.add(o);
            return this;
        }

        public JsonArray getJson() {
            return queryParams;
        }
    }

}