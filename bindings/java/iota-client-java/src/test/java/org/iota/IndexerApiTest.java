package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.types.*;
import org.iota.types.ids.AliasId;
import org.iota.types.ids.FoundryId;
import org.iota.types.ids.NftId;
import org.iota.types.ids.OutputId;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class IndexerApiTest extends ApiTest {

    @Test
    public void testGetBasicOutputIds() throws ClientException {
        for (OutputId outputId : client.getBasicOutputIds(new NodeIndexerApi.QueryParams()))
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIds() throws ClientException {
        for (OutputId outputId : client.getAliasOutputIds(new NodeIndexerApi.QueryParams()))
            System.out.println(outputId);
    }

    @Test
    public void testGetNftOutputIds() throws ClientException {
        for (OutputId outputId : client.getNftOutputIds(new NodeIndexerApi.QueryParams()))
            System.out.println(outputId);
    }

    @Test
    public void testGetFoundryOutputIds() throws ClientException {
        for (OutputId outputId : client.getFoundryOutputIds(new NodeIndexerApi.QueryParams()))
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIdByAliasId() throws ClientException {
        OutputId outputId = null;
        for (OutputId id : client.getAliasOutputIds(new NodeIndexerApi.QueryParams())) {
            if (client.getOutputWithMetadata(id).getKey().getJson().get("aliasId").getAsString().equals("0x0000000000000000000000000000000000000000000000000000000000000000")) {
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
        Output foundryOutput = client.getOutputWithMetadata(foundryOutputId).getKey();
        String aliasId = foundryOutput.getJson().get("unlockConditions").getAsJsonArray().get(0).getAsJsonObject().get("address").getAsJsonObject().get("aliasId").getAsString();
        int serialNumber = foundryOutput.getJson().get("serialNumber").getAsInt();
        int tokenScheme = foundryOutput.getJson().get("tokenScheme").getAsJsonObject().get("type").getAsInt();
        FoundryId foundryId = client.computeFoundryId(aliasId, serialNumber, tokenScheme);
        assertEquals(client.getFoundryOutputIdByFoundryId(foundryId), foundryOutputId);
    }

    @Test
    public void testGetNftOutputIdByNftId() throws ClientException {
        OutputId nftOutputId = client.getNftOutputIds(new NodeIndexerApi.QueryParams())[0];
        NftId nftId = client.computeNftId(nftOutputId);
        assertEquals(client.getNftOutputIdByNftId(nftId), nftOutputId);
    }

}
