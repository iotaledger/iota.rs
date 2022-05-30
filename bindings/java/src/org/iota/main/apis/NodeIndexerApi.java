package org.iota.main.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.node_indexer_api.OutputIdResponse;
import org.iota.main.types.responses.node_indexer_api.OutputIdsResponse;

public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public OutputIdsResponse getBasicOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);
        return (OutputIdsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BasicOutputIds", o.toString()));
    }

    public OutputIdsResponse getAliasOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);
        return (OutputIdsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputIds", o.toString()));
    }

    public OutputIdsResponse getNftOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);
        return (OutputIdsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputIds", o.toString()));
    }

    public OutputIdsResponse getFoundryOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);
        return (OutputIdsResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputIds", o.toString()));
    }

    public OutputIdResponse getAliasOutputId(String aliasId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasId", aliasId);
        return (OutputIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputId", o.toString()));
    }

    public OutputIdResponse getNftOutputId(String nftId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("nftId", nftId);
        return (OutputIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputId", o.toString()));
    }

    public OutputIdResponse getFoundryOutputId(String foundryId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("foundryId", foundryId);
        return (OutputIdResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputId", o.toString()));
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

