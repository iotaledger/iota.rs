package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.responses.ClientResponse;


public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public OutputId[] getBasicOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BasicOutputIds", o));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId[] getAliasOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputIds", o));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId[] getNftOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputIds", o));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId[] getFoundryOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputIds", o));
        JsonArray responsePayload = response.getPayload().getAsJsonArray();

        OutputId[] outputIds = new OutputId[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            outputIds[i] = new OutputId(responsePayload.get(i).getAsString());
        }

        return outputIds;
    }

    public OutputId getAliasOutputIdByAliasId(AliasId aliasId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasId", aliasId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputId", o));
        String responsePayload = response.getPayload().getAsString();

        return new OutputId(responsePayload);
    }

    public OutputId getNftOutputIdByNftId(NftId nftId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("nftId", nftId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputId", o));
        String responsePayload = response.getPayload().getAsString();

        return new OutputId(responsePayload);
    }

    public OutputId getFoundryOutputIdByFoundryId(FoundryId foundryId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("foundryId", foundryId.toString());

        ClientResponse response = callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputId", o));
        String responsePayload = response.getPayload().getAsString();

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