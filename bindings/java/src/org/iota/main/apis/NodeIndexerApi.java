package org.iota.main.apis;

import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.SuccessResponse;

public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public SuccessResponse getBasicOutputIds(QueryParams params) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "BasicOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public SuccessResponse getAliasOutputIds(QueryParams params) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public SuccessResponse getAliasOutputId(String aliasId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "AliasOutputId", "{\"aliasId\":\"" + aliasId + "\"}"));
    }

    public SuccessResponse getNftOutputIds(QueryParams params) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public SuccessResponse getNftOutputId(String nftId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "NftOutputId", "{\"nftId\":\"" + nftId + "\"}"));
    }

    public SuccessResponse getFoundryOutputIds(QueryParams params) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputIds", "{\"queryParameters\":" + params + "}"));
    }

    public SuccessResponse getFoundryOutputId(String foundryId) throws ClientException {
        return (SuccessResponse) callBaseApi(new ClientCommand(ClientCommand.CommandType.CallClientMethod, "FoundryOutputId", "{\"foundryId\":\"" + foundryId + "\"}"));
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

