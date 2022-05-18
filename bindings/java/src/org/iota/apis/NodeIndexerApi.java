package org.iota.apis;

import org.iota.ClientCommand;
import org.iota.ClientCommandType;
import org.iota.ClientConfig;
import org.iota.RustApi;

import java.util.List;

public class NodeIndexerApi extends BaseApi {

    public NodeIndexerApi(ClientConfig config) {
        super(config);
    }

    public String getBasicOutputIds(IndexerQueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"BasicOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getAliasOutputIds(IndexerQueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"AliasOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getAliasOutputId(String aliasId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"AliasOutputId\", \"data\": { \"aliasId\": \"" + aliasId + "\" }}"));
    }

    public String getNftOutputIds(IndexerQueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"NftOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getNftOutputId(String nftId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"NftOutputId\", \"data\": { \"nftId\": \"" + nftId + "\" }}"));
    }

    public String getFoundryOutputIds(IndexerQueryParams params) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"FoundryOutputIds\", \"data\": { \"queryParameters\": " + params + " }}"));
    }

    public String getFoundryOutputId(String foundryId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"FoundryOutputId\", \"data\": { \"foundryId\": \"" + foundryId + "\" }}"));
    }

}
