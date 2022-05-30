package org.iota.tests;

import org.iota.main.apis.NodeIndexerApi;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.node_indexer_api.OutputIdResponse;
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
        String aliasOutputId = client.getAliasOutputIds(new NodeIndexerApi.QueryParams()).getOutputIds()[0];
        String aliasId = client.computeAliasId(aliasOutputId).getAliasId();
        OutputIdResponse r = client.getAliasOutputId(aliasId);
        System.out.println(r.getOutputId());
    }

    @Test
    public void testGetNftOutputId() throws ClientException {
        String nftOutputId = client.getNftOutputIds(new NodeIndexerApi.QueryParams()).getOutputIds()[0];
        String nftId = client.computeNftId(nftOutputId).getNftId();
        OutputIdResponse r = client.getNftOutputId(nftId);
        System.out.println(r.getOutputId());
    }



}
