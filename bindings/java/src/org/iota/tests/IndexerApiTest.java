package org.iota.tests;

import org.iota.main.apis.NodeIndexerApi;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.node_indexer_api.AliasOutputIdResponse;
import org.iota.main.types.responses.node_indexer_api.FoundryOutputIdResponse;
import org.iota.main.types.responses.node_indexer_api.NftOutputIdResponse;
import org.iota.main.types.responses.node_indexer_api.OutputIdsResponse;
import org.junit.jupiter.api.Test;

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
        OutputIdsResponse r = client.getNftOutputIds(new NodeIndexerApi.QueryParams());
        for(String outputId: r.getOutputIds())
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputId() throws ClientException {
        AliasOutputIdResponse r = client.getAliasOutputId("0x1505ec099896ab05d9e08fbc7101ae4dff0093b3943b28f789ed2ca728bcc8d6");
        System.out.println(r.getAliasOutputId());
    }

    @Test
    public void testGetNftOutputId() throws ClientException {
        NftOutputIdResponse r = client.getNftOutputId("0x1505ec099896ab05d9e08fbc7101ae4dff0093b3943b28f789ed2ca728bcc8d6");
        System.out.println(r.getNftOutputId());
    }

    @Test
    public void testFoundryOutputId() throws ClientException {
        FoundryOutputIdResponse r = client.getFoundryOutputId("0x081505ec099896ab05d9e08fbc7101ae4dff0093b3943b28f789ed2ca728bcc8d60100000000");
        System.out.println(r.getFoundryOutputId());
    }

}
