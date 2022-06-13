package org.iota.tests;

import org.iota.main.apis.NodeIndexerApi;
import org.iota.main.types.ClientException;
import org.iota.main.types.Output;
import org.iota.main.types.responses.node_indexer_api.OutputIdResponse;
import org.iota.main.types.responses.node_indexer_api.OutputIdsResponse;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class IndexerApiTest extends ApiTest {

    @Test
    public void testGetBasicOutputIds() throws ClientException {
        OutputIdsResponse r = client.getBasicOutputIds(new NodeIndexerApi.QueryParams());
        for(String outputId: r.getOutputIds())
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIds() throws ClientException {
        OutputIdsResponse r = client.getAliasOutputIds(new NodeIndexerApi.QueryParams());
        for(String outputId: r.getOutputIds())
            System.out.println(outputId);
    }

    @Test
    public void testGetNftOutputIds() throws ClientException {
        OutputIdsResponse r = client.getNftOutputIds(new NodeIndexerApi.QueryParams());
        for(String outputId: r.getOutputIds())
            System.out.println(outputId);
    }

    @Test
    public void testGetFoundryOutputIds() throws ClientException {
        OutputIdsResponse r = client.getFoundryOutputIds(new NodeIndexerApi.QueryParams());
        for(String outputId: r.getOutputIds())
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIdByAliasId() throws ClientException {
        String outputId = null;
        for(String id: client.getAliasOutputIds(new NodeIndexerApi.QueryParams()).getOutputIds()) {
            if(client.getOutputWithMetadata(id).getOutput().getAsJsonObject().get("aliasId").getAsString().equals("0x0000000000000000000000000000000000000000000000000000000000000000")) {
                outputId = id;
                break;
            }
        }
        String aliasId = client.computeAliasId(outputId).getAliasId();
        OutputIdResponse r = client.getAliasOutputIdByAliasId(aliasId);
        assertEquals(r.getOutputId(),outputId);
    }

    @Test
    public void testGetFoundryOutputIdByFoundryId() throws ClientException {
        String foundryOutputId = client.getFoundryOutputIds(new NodeIndexerApi.QueryParams()).getOutputIds()[0];
        Output foundryOutput = client.getOutputWithMetadata(foundryOutputId).getOutput();
        String aliasId = foundryOutput.getAsJsonObject().get("unlockConditions").getAsJsonArray().get(0).getAsJsonObject().get("address").getAsJsonObject().get("aliasId").getAsString();
        int serialNumber = foundryOutput.getAsJsonObject().get("serialNumber").getAsInt();
        int tokenScheme = foundryOutput.getAsJsonObject().get("tokenScheme").getAsJsonObject().get("type").getAsInt();
        String foundryId = client.computeFoundryId(aliasId, serialNumber, tokenScheme).getFoundryId();
        OutputIdResponse r = client.getFoundryOutputIdByFoundryId(foundryId);
        assertEquals(r.getOutputId(),foundryOutputId);
    }

    @Test
    public void testGetNftOutputIdByNftId() throws ClientException {
        String nftOutputId = client.getNftOutputIds(new NodeIndexerApi.QueryParams()).getOutputIds()[0];
        String nftId = client.computeNftId(nftOutputId).getNftId();
        OutputIdResponse r = client.getNftOutputIdByNftId(nftId);
        assertEquals(r.getOutputId(),nftOutputId);
    }


}
