// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import org.iota.apis.NodeIndexerApi;
import org.iota.types.*;
import org.iota.types.ids.*;

import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.secret.BuildBlockOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class IndexerApiTest extends ApiTest {

    @Test
    public void testGetBasicOutputIds() throws ClientException {
        for (OutputId outputId : client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", generateAddress(DEFAULT_DEVELOPMENT_MNEMONIC))))
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIds() throws ClientException {
        for (OutputId outputId : client.getAliasOutputIds(new NodeIndexerApi.QueryParams().withParam("issuer", generateAddress(DEFAULT_DEVELOPMENT_MNEMONIC))))
            System.out.println(outputId);
    }

    @Test
    public void testGetNftOutputIds() throws ClientException {
        for (OutputId outputId : client.getNftOutputIds(new NodeIndexerApi.QueryParams().withParam("address", generateAddress(DEFAULT_DEVELOPMENT_MNEMONIC))))
            System.out.println(outputId);
    }

    @Test
    public void testGetFoundryOutputIds() throws ClientException {
        JsonObject json = new Gson().fromJson(
                "{\"type\":4,\"amount\":\"1000000\",\"aliasId\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"stateIndex\":0,\"foundryCounter\":0,\"unlockConditions\":[{\"type\":4,\"address\":{\"type\":0,\"pubKeyHash\":\"0x4cfde0600797ae07d19d67d78910e70950bfdaf716f0035e9a30b97828aaf6a2\"}},{\"type\":5,\"address\":{\"type\":0,\"pubKeyHash\":\"0x4cfde0600797ae07d19d67d78910e70950bfdaf716f0035e9a30b97828aaf6a2\"}}],\"features\":[{\"type\":0,\"address\":{\"type\":0,\"pubKeyHash\":\"0x4cfde0600797ae07d19d67d78910e70950bfdaf716f0035e9a30b97828aaf6a2\"}},{\"type\":2,\"data\":\"0x010203\"}],\"immutableFeatures\":[{\"type\":1,\"address\":{\"type\":0,\"pubKeyHash\":\"0x4cfde0600797ae07d19d67d78910e70950bfdaf716f0035e9a30b97828aaf6a2\"}}]}",
                JsonObject.class
        );
        Output o = new Output(json);

        client.buildAndPostBlock(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new BuildBlockOptions().withOutputs(new Output[] { o }));

        for (OutputId outputId : client.getFoundryOutputIds(new NodeIndexerApi.QueryParams().withParam("aliasAddress", "rms1pz022kd0zjmu4dms8whgnfus3ph347cmr8th3kk5g2qn7sl5xzfcj33gfu4")))
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIdByAliasId() throws ClientException {
        OutputId outputId = null;
        for (OutputId id : client.getAliasOutputIds(new NodeIndexerApi.QueryParams())) {
            if (client.getOutput(id).getKey().toJson().get("aliasId").getAsString().equals("0x0000000000000000000000000000000000000000000000000000000000000000")) {
                outputId = id;
                break;
            }
        }
        AliasId aliasId = client.computeAliasId(outputId);
        assertEquals(client.getAliasOutputIdByAliasId(aliasId), outputId);
    }

    @Test
    public void testGetFoundryOutputIdByFoundryId() throws ClientException {
        OutputId foundryOutputId = client.getFoundryOutputIds(new NodeIndexerApi.QueryParams())[0];
        Output foundryOutput = client.getOutput(foundryOutputId).getKey();
        String aliasId = foundryOutput.toJson().get("unlockConditions").getAsJsonArray().get(0).getAsJsonObject().get("address").getAsJsonObject().get("aliasId").getAsString();
        int serialNumber = foundryOutput.toJson().get("serialNumber").getAsInt();
        int tokenScheme = foundryOutput.toJson().get("tokenScheme").getAsJsonObject().get("type").getAsInt();
        FoundryId foundryId = client.computeFoundryId(aliasId, serialNumber, tokenScheme);
        assertEquals(client.getFoundryOutputIdByFoundryId(foundryId), foundryOutputId);
    }

    @Test
    public void testGetNftOutputIdByNftId() throws ClientException {
        OutputId nftOutputId = null;
        for (OutputId id : client.getNftOutputIds(new NodeIndexerApi.QueryParams())) {
            if (client.getOutput(id).getKey().toJson().get("nftId").getAsString().equals("0x0000000000000000000000000000000000000000000000000000000000000000")) {
                nftOutputId = id;
                break;
            }
        }
        NftId nftId = client.computeNftId(nftOutputId);
        assertEquals(client.getNftOutputIdByNftId(nftId), nftOutputId);
    }

}
