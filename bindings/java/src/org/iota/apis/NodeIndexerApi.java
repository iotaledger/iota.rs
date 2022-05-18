package org.iota.apis;

import org.iota.ClientCommand;
import org.iota.ClientCommandType;
import org.iota.ClientConfig;
import org.iota.RustApi;

public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig config) {
        super(config);
    }

    public String getBasicOutputIds(QueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"BasicOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getAliasOutputIds(QueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"AliasOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getAliasOutputId(String aliasId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"AliasOutputId\", \"data\": { \"aliasId\": \"" + aliasId + "\" }}"));
    }

    public String getNftOutputIds(QueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"NftOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getNftOutputId(String nftId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"NftOutputId\", \"data\": { \"nftId\": \"" + nftId + "\" }}"));
    }

    public String getFoundryOutputIds(QueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"FoundryOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getFoundryOutputId(String foundryId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"FoundryOutputId\", \"data\": { \"foundryId\": \"" + foundryId + "\" }}"));
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

