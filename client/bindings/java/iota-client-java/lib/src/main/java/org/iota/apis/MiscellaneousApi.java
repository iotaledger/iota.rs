// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.*;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.BlockId;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.output_builder.BasicOutputBuilderParams;
import org.iota.types.output_builder.FoundryOutputBuilderParams;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.responses.ProtocolParametersResponse;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.BuildBlockOptions;
import org.iota.types.secret.SecretManager;

import java.util.AbstractMap;
import java.util.Map;

public class MiscellaneousApi {

    private NativeApi nativeApi;

    public MiscellaneousApi(NativeApi nativeApi) throws InitializeClientException {
        this.nativeApi = nativeApi;
    }

    public Output buildAliasOutput(
            AliasOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("buildAliasOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public Output buildBasicOutput(
            BasicOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("buildBasicOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public Output buildFoundryOutput(
            FoundryOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("buildFoundryOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public Output buildNftOutput(
            NftOutputBuilderParams params
    ) throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("buildNftOutput", params.getJson()));

        return new Output(responsePayload);
    }

    public String[] generateAddresses(SecretManager secretManager, GenerateAddressesOptions generateAddressesOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("options", generateAddressesOptions.getJson());

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("generateAddresses", o));

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

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("buildAndPostBlock", o));

        BlockId blockId = new BlockId(responsePayload.get(0).getAsString());
        Block block = new Block(responsePayload.get(1).getAsJsonObject());

        return new AbstractMap.SimpleEntry<>(blockId, block);
    }

    public Node getNode() throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getNode"));
        return new Node(responsePayload);
    }

    public JsonObject getNetworkInfo() throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getNetworkInfo"));
        return responsePayload;
    }

    public int getNetworkId() throws ClientException {
        Integer responsePayload = nativeApi.sendCommand(new ClientCommand("getNetworkId")).getAsInt();
        return responsePayload;
    }

    public String getBech32Hrp() throws ClientException {
        String responsePayload = nativeApi.sendCommand(new ClientCommand("getBech32Hrp")).getAsString();
        return responsePayload;
    }

    public int getMinPowScore() throws ClientException {
        Integer responsePayload = nativeApi.sendCommand(new ClientCommand("getMinPowScore")).getAsInt();
        return responsePayload;
    }

    public int getTipsInterval() throws ClientException {
        Integer responsePayload = nativeApi.sendCommand(new ClientCommand("getTipsInterval")).getAsInt();
        return responsePayload;
    }

    public boolean getLocalPow() throws ClientException {
        Boolean responsePayload = nativeApi.sendCommand(new ClientCommand("getLocalPow")).getAsBoolean();
        return responsePayload;
    }

    public boolean isFallbackToLocalPow() throws ClientException {
        Boolean responsePayload = nativeApi.sendCommand(new ClientCommand("getFallbackToLocalPow")).getAsBoolean();
        return responsePayload;
    }

    public Node[] getUnhealthyNodes() throws ClientException {
        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("unhealthyNodes"));

        Node[] nodes = new Node[responsePayload.size()];
        for (int i = 0; i < responsePayload.size(); i++) {
            nodes[i] = new Node(responsePayload.get(i).getAsJsonObject());
        }

        return nodes;
    }

    public LedgerNanoStatus getLedgerNanoStatus(boolean isSimulator) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("ledgerNano", isSimulator);

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getLedgerNanoStatus", o));

        return new LedgerNanoStatus(responsePayload);
    }

    public PreparedTransactionData prepareTransaction(SecretManager secretManager, BuildBlockOptions buildBlockOptions) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("buildBlockOptions", buildBlockOptions.getJson());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("prepareTransaction", o));

        return new PreparedTransactionData(responsePayload);
    }

    public TransactionPayload signTransaction(SecretManager secretManager, PreparedTransactionData preparedTransactionData) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.add("preparedTransactionData", preparedTransactionData.toJson());

        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("signTransaction", o));

        return new TransactionPayload(responsePayload);
    }

    public void storeMnemonic(SecretManager secretManager, String mnemonic) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("secretManager", secretManager.getJson());
        o.addProperty("mnemonic", mnemonic);

        nativeApi.sendCommand(new ClientCommand("storeMnemonic", o));
    }

    public Map.Entry<BlockId, Block> postBlockPayload(BlockPayload payload) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("payload", payload.toJson());

        JsonArray responsePayload = (JsonArray) nativeApi.sendCommand(new ClientCommand("postBlockPayload", o));

        BlockId blockId = new BlockId(responsePayload.get(0).getAsString());
        Block block = new Block(responsePayload.get(1).getAsJsonObject());

        return new AbstractMap.SimpleEntry<>(blockId, block);
    }

    /**
     * Returns the protocol parameters.
     */
    public ProtocolParametersResponse getProtocolParameters() throws ClientException {
        JsonObject responsePayload = (JsonObject) nativeApi.sendCommand(new ClientCommand("getProtocolParameters"));
        return new ProtocolParametersResponse(responsePayload);
    }

}

