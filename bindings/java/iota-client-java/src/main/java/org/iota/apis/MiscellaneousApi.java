// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.ids.BlockId;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.output_builder.BasicOutputBuilderParams;
import org.iota.types.output_builder.FoundryOutputBuilderParams;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.BuildBlockOptions;
import org.iota.types.secret.SecretManager;

import java.util.AbstractMap;
import java.util.Map;

public class MiscellaneousApi extends BaseApi {

    public MiscellaneousApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public Output buildAliasOutput(
            AliasOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("BuildAliasOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public Output buildBasicOutput(
            BasicOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("BuildBasicOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public Output buildFoundryOutput(
            FoundryOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("BuildFoundryOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public Output buildNftOutput(
            NftOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("BuildNftOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public String[] generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("options", generateAddressesOptions.getJson());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("GenerateAddresses", o));

        String[] addresses = new String[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            addresses[i] = responsePayload.get(i).getAsString();
        }

        return addresses;
    }

    public Map.Entry<BlockId, Block> buildAndPostBlock(SecretManager secretManager, BuildBlockOptions options) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager != null ? secretManager.getJson() : null);
        o.add("options", options != null ? options.getJson() : null);

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("BuildAndPostBlock", o));

        BlockId blockId = new BlockId(responsePayload.get(0).getAsString());
        Block block = new Block(responsePayload.get(1).getAsJsonObject());

        return new AbstractMap.SimpleEntry<>(blockId, block);
    }

    public Node getNode() throws ClientException {
        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("GetNode"));
        return new Node(responsePayload);
    }

    public JsonObject getNetworkInfo() throws ClientException {
        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("GetNetworkInfo"));
        return responsePayload;
    }

    public int getNetworkId() throws ClientException {
        Integer responsePayload = callBaseApi(new ClientCommand("GetNetworkId")).getAsInt();
        return responsePayload;
    }

    public String getBech32Hrp() throws ClientException {
        String responsePayload = callBaseApi(new ClientCommand("GetBech32Hrp")).getAsString();
        return responsePayload;
    }

    public int getMinPowScore() throws ClientException {
        Integer responsePayload = callBaseApi(new ClientCommand("GetMinPowScore")).getAsInt();
        return responsePayload;
    }

    public int getTipsInterval() throws ClientException {
        Integer responsePayload = callBaseApi(new ClientCommand("GetTipsInterval")).getAsInt();
        return responsePayload;
    }

    public boolean getLocalPow() throws ClientException {
        Boolean responsePayload = callBaseApi(new ClientCommand("GetLocalPow")).getAsBoolean();
        return responsePayload;
    }

    public boolean isFallbackToLocalPow() throws ClientException {
        Boolean responsePayload = callBaseApi(new ClientCommand("GetFallbackToLocalPow")).getAsBoolean();
        return responsePayload;
    }

    public Node[] getUnsyncedNodes() throws ClientException {
        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("UnsyncedNodes"));

        Node[] nodes = new Node[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            nodes[i] = new Node(responsePayload.get(i).getAsJsonObject());
        }

        return nodes;
    }

    public LedgerNanoStatus getLedgerNanoStatus(boolean isSimulator) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("LedgerNano", isSimulator);

        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("GetLedgerNanoStatus", o));

        return new LedgerNanoStatus(responsePayload);
    }

    public PreparedTransactionData prepareTransaction(SecretManager secretManager, BuildBlockOptions buildBlockOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("buildBlockOptions", buildBlockOptions.getJson());

        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("PrepareTransaction", o));

        return new PreparedTransactionData(responsePayload);
    }

    public TransactionPayload signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("preparedTransactionData", preparedTransactionData.toJson());

        JsonObject responsePayload = (JsonObject) callBaseApi(new ClientCommand("SignTransaction", o));

        return new TransactionPayload(responsePayload);
    }

    public void storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.addProperty("mnemonic", mnemonic);

        callBaseApi(new ClientCommand("StoreMnemonic", o));
    }

    public Map.Entry<BlockId, Block> postBlockPayload(BlockPayload payload) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("payload", payload.toJson());

        JsonArray responsePayload = (JsonArray) callBaseApi(new ClientCommand("PostBlockPayload", o));

        BlockId blockId = new BlockId(responsePayload.get(0).getAsString());
        Block block = new Block(responsePayload.get(1).getAsJsonObject());

        return new AbstractMap.SimpleEntry<>(blockId, block);
    }

}

