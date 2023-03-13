// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.AliasId;
import org.iota.types.ids.FoundryId;
import org.iota.types.ids.NftId;
import org.iota.types.ids.OutputId;
import org.iota.types.responses.OutputIdsResponse;

public class NodeIndexerApi {

    private NativeApi nativeApi;

    public NodeIndexerApi(NativeApi nativeApi) throws InitializeClientException {
        this.nativeApi = nativeApi;
    }

    public OutputIdsResponse getBasicOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("basicOutputIds", o));

        return new OutputIdsResponse(responsePayload);
    }

    public OutputIdsResponse getAliasOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("aliasOutputIds", o));

        return new OutputIdsResponse(responsePayload);
    }

    public OutputIdsResponse getNftOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("nftOutputIds", o));

        return new OutputIdsResponse(responsePayload);
    }

    public OutputIdsResponse getFoundryOutputIds(QueryParams params) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("queryParameters", params.queryParams);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("foundryOutputIds", o));

        return new OutputIdsResponse(responsePayload);
    }

    public OutputId getAliasOutputIdByAliasId(AliasId aliasId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasId", aliasId.toString());

        String responsePayload = nativeApi.sendCommand(new ClientCommand("aliasOutputId", o)).getAsString();

        return new OutputId(responsePayload);
    }

    public OutputId getNftOutputIdByNftId(NftId nftId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("nftId", nftId.toString());

        String responsePayload = nativeApi.sendCommand(new ClientCommand("nftOutputId", o)).getAsString();

        return new OutputId(responsePayload);
    }

    public OutputId getFoundryOutputIdByFoundryId(FoundryId foundryId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("foundryId", foundryId.toString());

        String responsePayload = nativeApi.sendCommand(new ClientCommand("foundryOutputId", o)).getAsString();

        return new OutputId(responsePayload);
    }


    public static class QueryParams {

        private JsonArray queryParams = new JsonArray();

        public QueryParams withParam(String name, Boolean value) {
            withParam(name, new JsonPrimitive(value));
            return this;
        }

        public QueryParams withParam(String name, Number value) {
            withParam(name, new JsonPrimitive(value));
            return this;
        }

        public QueryParams withParam(String name, String value) {
            withParam(name, new JsonPrimitive(value));
            return this;
        }

        public QueryParams withParam(String name, Character value) {
            withParam(name, new JsonPrimitive(value));
            return this;
        }

        private void withParam(String name, JsonPrimitive value) {
            JsonObject o = new JsonObject();
            o.add(name, value);
            queryParams.add(o);
        }

        public JsonArray getJson() {
            return queryParams;
        }
    }

}