// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.apis;

import com.google.gson.JsonObject;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.TransactionPayload;
import org.iota.types.ids.*;

public class UtilsApi extends BaseApi {

    public UtilsApi(ClientConfig clientConfig) {
        super(clientConfig);
    }

    public String bech32ToHex(String bech32) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("bech32", bech32);

        String responsePayload = callBaseApi(new ClientCommand("bech32ToHex", o)).getAsString();

        return responsePayload;
    }

    public String hexToBech32(String hex, String bech32) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("hex", hex);
        o.addProperty("bech32Hrp", bech32);

        String responsePayload = callBaseApi(new ClientCommand("hexToBech32", o)).getAsString();

        return responsePayload;
    }

    public String hexPublicKeyToBech32Address(String hex, String bech32) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("hex", hex);
        o.addProperty("bech32", bech32);

        String responsePayload = callBaseApi(new ClientCommand("hexPublicKeyToBech32Address", o)).getAsString();

        return responsePayload;
    }

    public String parseBech32Address(String address) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("address", address);

        String responsePayload = callBaseApi(new ClientCommand("parseBech32Address", o)).getAsString();

        return responsePayload;
    }

    public boolean isAddressValid(String address) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("address", address);

        Boolean responsePayload = callBaseApi(new ClientCommand("isAddressValid", o)).getAsBoolean();

        return responsePayload;
    }

    public String generateMnemonic() throws ClientException {
        String responsePayload = callBaseApi(new ClientCommand("generateMnemonic")).getAsString();
        return responsePayload;
    }

    public String mnemonicToHexSeed(String mnemonic) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("mnemonic", mnemonic);

        String responsePayload = callBaseApi(new ClientCommand("mnemonicToHexSeed", o)).getAsString();
        return responsePayload;
    }

    public BlockId computeBlockId(Block block) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("block", block.toJson());

        String responsePayload = callBaseApi(new ClientCommand("blockId", o)).getAsString();

        return new BlockId(responsePayload);
    }

    public TransactionId getTransactionId(TransactionPayload payload) throws ClientException {
        JsonObject o = new JsonObject();
        o.add("payload", payload.toJson());

        String responsePayload = callBaseApi(new ClientCommand("transactionId", o)).getAsString();

        return new TransactionId(responsePayload);
    }

    public String requestFundsFromFaucet(String faucetUrl, String address) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("url", faucetUrl);
        o.addProperty("address", address);

        String responsePayload = callBaseApi(new ClientCommand("faucet", o)).getAsString();

        return responsePayload;
    }

    public AliasId computeAliasId(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        String responsePayload = callBaseApi(new ClientCommand("computeAliasId", o)).getAsString();

        return new AliasId(responsePayload);
    }

    public NftId computeNftId(OutputId outputId) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("outputId", outputId.toString());

        String responsePayload = callBaseApi(new ClientCommand("computeNftId", o)).getAsString();

        return new NftId(responsePayload);
    }

    public FoundryId computeFoundryId(String aliasAddress, int serialNumber, int tokenScheme) throws ClientException {
        JsonObject o = new JsonObject();
        o.addProperty("aliasAddress", aliasAddress);
        o.addProperty("serialNumber", serialNumber);
        o.addProperty("tokenSchemeKind", tokenScheme);

        String responsePayload = callBaseApi(new ClientCommand("computeFoundryId", o)).getAsString();

        return new FoundryId(responsePayload);
    }
}

