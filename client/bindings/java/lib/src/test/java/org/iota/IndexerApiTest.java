// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.types.*;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.NoFundsReceivedFromFaucetException;
import org.iota.types.ids.*;
import org.iota.types.output_builder.AliasOutputBuilderParams;
import org.iota.types.output_builder.FoundryOutputBuilderParams;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.secret.*;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.util.LinkedHashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class IndexerApiTest extends ApiTest {

    @Test
    public void testGetBasicOutputIds() throws ClientException, NoFundsReceivedFromFaucetException {
        String address = generateAddress(client.generateMnemonic());
        client.requestTestFundsFromFaucet(address);
        for (OutputId outputId : client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address)).getItems())
            System.out.println(outputId);
    }

    @Test
    public void testGetAliasOutputIds() throws ClientException, InterruptedException, NoFundsReceivedFromFaucetException {
        SecretManager s = new MnemonicSecretManager(client.generateMnemonic());
        String address = client.generateAddresses(s, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0];
        client.requestTestFundsFromFaucet(address);
        String hexAddress = client.bech32ToHex(address);

        AliasId aliasId = new AliasId("0x0000000000000000000000000000000000000000000000000000000000000000");
        UnlockCondition[] unlockConditions = new UnlockCondition[]{
                new UnlockCondition("{ type: 4, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                new UnlockCondition("{ type: 5, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
        };
        AliasOutputBuilderParams params = new AliasOutputBuilderParams()
                .withAliasId(aliasId)
                .withUnlockConditions(unlockConditions);

        Output aliasOutput = client.buildAliasOutput(params);

        Map.Entry<BlockId, Block> entry = client.buildAndPostBlock(s, new BuildBlockOptions().withOutputs(new Output[] { aliasOutput }));
        client.retryUntilIncluded(entry.getKey(), 2, 15);

        for (OutputId outputId : client.getAliasOutputIds(new NodeIndexerApi.QueryParams().withParam("governor", address)).getItems())
            System.out.println(outputId);
    }

    @Test
    public void testGetNftOutputIds() throws ClientException, NoFundsReceivedFromFaucetException, InterruptedException {
        SecretManager s = new MnemonicSecretManager(client.generateMnemonic());
        String address = client.generateAddresses(s, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0];
        client.requestTestFundsFromFaucet(address);
        String hexAddress = client.bech32ToHex(address);

        NftId nftId = new NftId("0x0000000000000000000000000000000000000000000000000000000000000000");
        UnlockCondition[] unlockConditions = new UnlockCondition[]{
                new UnlockCondition("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
        };
        NftOutputBuilderParams params = new NftOutputBuilderParams()
                .withNftId(nftId)
                .withUnlockConditions(unlockConditions);

        Output aliasOutput = client.buildNftOutput(params);

        Map.Entry<BlockId, Block> entry = client.buildAndPostBlock(s, new BuildBlockOptions().withOutputs(new Output[] { aliasOutput }));
        client.retryUntilIncluded(entry.getKey(), 2, 15);

        for (OutputId outputId : client.getNftOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address)).getItems())
            System.out.println(outputId);
    }

    @Test
    public void testGetFoundryOutputIds() throws ClientException, NoFundsReceivedFromFaucetException, InterruptedException {
        SecretManager s = new MnemonicSecretManager(client.generateMnemonic());
        String address = client.generateAddresses(s, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0];
        client.requestTestFundsFromFaucet(address);
        String hexAddress = client.bech32ToHex(address);

        // Build an Alias Output
        AliasOutputBuilderParams p = new AliasOutputBuilderParams()
                .withAliasId(new AliasId("0x0000000000000000000000000000000000000000000000000000000000000000"))
                .withUnlockConditions(new UnlockCondition[]{
                        new UnlockCondition("{ type: 4, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                        new UnlockCondition("{ type: 5, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
                });

        Map.Entry<BlockId, Block> response = client.buildAndPostBlock(s, new BuildBlockOptions().withOutputs(new Output[] { client.buildAliasOutput(p) }));
        client.retryUntilIncluded(response.getKey(), 2, 15);
        TransactionId transactionId = client.getTransactionId(new TransactionPayload(response.getValue().toJson().get("payload").getAsJsonObject()));

        // Build the Foundry Output
        client.requestTestFundsFromFaucet(address);
        AliasId aliasId = client.computeAliasId(new OutputId(transactionId + "0000" ));
        int serialNumber = 1;
        TokenScheme tokenScheme = new TokenScheme("{ type: 0, meltedTokens: '0x0', mintedTokens: '0x32', maximumSupply: '0x64' }");
        UnlockCondition[] unlockConditions = new UnlockCondition[]{new UnlockCondition("{ type: 6, address: { type: 8, aliasId: " + aliasId + " } }")};
        FoundryOutputBuilderParams params = new FoundryOutputBuilderParams()
                .withSerialNumber(serialNumber)
                .withTokenScheme(tokenScheme)
                .withUnlockConditions(unlockConditions);
        Output foundryOutput = client.buildFoundryOutput(params);

        // Build the new foundry output
        Output newAliasOutput = client.buildAliasOutput(new AliasOutputBuilderParams()
                .withAliasId(aliasId)
                .withFoundryCounter(1)
                .withStateIndex(1)
                .withUnlockConditions(new UnlockCondition[]{
                        new UnlockCondition("{ type: 4, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
                        new UnlockCondition("{ type: 5, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
                }));

        // Create the transaction and use the outputs
        Map.Entry<BlockId, Block> entry = client.buildAndPostBlock(s, new BuildBlockOptions()
                .withOutputs(new Output[] {
                        foundryOutput,
                        newAliasOutput
                })
        );

        client.retryUntilIncluded(entry.getKey(), 2, 15);

        for (OutputId outputId : client.getFoundryOutputIds(new NodeIndexerApi.QueryParams().withParam("aliasAddress", client.aliasIdToBech32(aliasId, client.getBech32Hrp()))).getItems())
            System.out.println(outputId);
    }

    @Test
    @Disabled
    public void testGetAliasOutputIdByAliasId() throws ClientException {
        OutputId outputId = null;
        for (OutputId id : client.getAliasOutputIds(new NodeIndexerApi.QueryParams()).getItems()) {
            if (client.getOutput(id).getKey().toJson().get("aliasId").getAsString().equals("0x0000000000000000000000000000000000000000000000000000000000000000")) {
                outputId = id;
                break;
            }
        }
        AliasId aliasId = client.computeAliasId(outputId);
        assertEquals(client.getAliasOutputIdByAliasId(aliasId), outputId);
    }

    @Test
    @Disabled
    public void testGetFoundryOutputIdByFoundryId() throws ClientException {
        OutputId foundryOutputId = client.getFoundryOutputIds(new NodeIndexerApi.QueryParams()).getItems()[0];
        Output foundryOutput = client.getOutput(foundryOutputId).getKey();
        String aliasId = foundryOutput.toJson().get("unlockConditions").getAsJsonArray().get(0).getAsJsonObject().get("address").getAsJsonObject().get("aliasId").getAsString();
        int serialNumber = foundryOutput.toJson().get("serialNumber").getAsInt();
        int tokenScheme = foundryOutput.toJson().get("tokenScheme").getAsJsonObject().get("type").getAsInt();
        FoundryId foundryId = client.computeFoundryId(aliasId, serialNumber, tokenScheme);
        assertEquals(client.getFoundryOutputIdByFoundryId(foundryId), foundryOutputId);
    }

    @Test
    @Disabled
    public void testGetNftOutputIdByNftId() throws ClientException {
        OutputId nftOutputId = null;
        for (OutputId id : client.getNftOutputIds(new NodeIndexerApi.QueryParams()).getItems()) {
            if (client.getOutput(id).getKey().toJson().get("nftId").getAsString().equals("0x0000000000000000000000000000000000000000000000000000000000000000")) {
                nftOutputId = id;
                break;
            }
        }
        NftId nftId = client.computeNftId(nftOutputId);
        assertEquals(client.getNftOutputIdByNftId(nftId), nftOutputId);
    }

}
