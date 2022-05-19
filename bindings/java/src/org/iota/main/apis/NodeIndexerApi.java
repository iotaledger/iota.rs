package org.iota.main.apis;

import org.iota.main.ClientConfig;

public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String getBasicOutputIds(QueryParams params) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BasicOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public String getAliasOutputIds(QueryParams params) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public String getAliasOutputId(String aliasId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputId", "{\"aliasId\":\"" + aliasId + "\"}"));
    }

    public String getNftOutputIds(QueryParams params) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public String getNftOutputId(String nftId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputId", "{\"nftId\":\"" + nftId + "\"}"));
    }

    public String getFoundryOutputIds(QueryParams params) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public String getFoundryOutputId(String foundryId) {
        return callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputId", "{\"foundryId\":\"" + foundryId + "\"}"));
    }

    public static class QueryParams {

        private String json;

        public QueryParams() {
            this.json = "[]";
        }

        public QueryParams(String json) {
            this.json = json;
        }

        @Override
        public String toString() {
            return json;
        }
    }

}

