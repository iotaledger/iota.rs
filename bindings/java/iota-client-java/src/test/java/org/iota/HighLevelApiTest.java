// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.types.Block;
import org.iota.types.ClientException;
import org.iota.types.UtxoInput;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.OutputId;
import org.iota.types.secret.*;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertThrows;

public class HighLevelApiTest extends ApiTest {

    @Test
    public void testGetOutputs() throws ClientException {
        OutputId[] outputs = client.getBasicOutputIds(new NodeIndexerApi.QueryParams());
        // Limit outputs to 5 to not send thousands of requests
        OutputId[] limited_outputs = Arrays.copyOfRange(outputs, 0, 5);
        for (Map.Entry e : client.getOutputs(limited_outputs)) {
            System.out.println(e.getKey());
        }
    }

    @Test
    public void testTryGetOutputs() throws ClientException {
        OutputId[] outputs = client.getBasicOutputIds(new NodeIndexerApi.QueryParams());
        // Limit outputs to 5 to not send thousands of requests
        OutputId[] limited_outputs = Arrays.copyOfRange(outputs, 0, 5);
        for (Map.Entry e : client.tryGetOutputs(limited_outputs)) {
            System.out.println(e.getKey());
        }
    }

    @Test
    public void testFindBlocks() throws ClientException {
        BlockId[] blockIds = client.getTips();
        for (Block b : client.findBlocks(blockIds)) {
            System.out.println(b);
        }
    }

    @Test
    public void testRetryBlock() {
        assertThrows(
                ClientException.class,
                () -> {
                    Map.Entry<BlockId, Block> ret = client.retry(client.getTips()[0]);
                    System.out.println(ret.getKey());
                    System.out.println(ret.getValue());
                }
        );
    }

    @Test
    public void testRetryUntilIncludedBlock() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(client.generateMnemonic());
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 2)));
        requestFundsFromFaucet(addresses[0]);
        BuildBlockOptions.ClientBlockBuilderOutputAddress output = new BuildBlockOptions.ClientBlockBuilderOutputAddress(addresses[1], Integer.toString(1000000));
        Map.Entry<BlockId, Block> entry = client.buildAndPostBlock(secretManager, new BuildBlockOptions().withOutput(output));
        LinkedHashMap<BlockId, Block> ret = client.retryUntilIncluded(entry.getKey(), 2, 15);
        for(BlockId i : ret.keySet())
            System.out.println(i);
    }

    @Test
    public void testConsolidateFunds() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String address = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)))[0];
        requestFundsFromFaucet(address);
        String consolidatedAddress = client.consolidateFunds(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)));
        System.out.println(consolidatedAddress);
    }

    @Test
    public void testFindInputs() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 5)));
        requestFundsFromFaucet(addresses[0]);
        UtxoInput[] inputs = client.findInputs(addresses, 1000);
        for (UtxoInput id : inputs)
            System.out.println(id);
    }

    @Test
    public void testFindOutputs() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 5)));
        requestFundsFromFaucet(addresses[0]);
        for (Map.Entry e : client.findOutputs(new OutputId[]{}, addresses)) {
            System.out.println(e.getKey());
        }
    }

    @Test
    public void testReattach() {
        assertThrows(
                ClientException.class,
                () -> {
                    Map.Entry<BlockId, Block> entry = client.reattach(client.getTips()[0]);
                    System.out.println(entry.getKey());
                    System.out.println(entry.getValue());
                }
        );
    }

    @Test
    public void testReattachUnchecked() throws ClientException {
        Map.Entry<BlockId, Block> entry = client.reattachUnchecked(client.getTips()[0]);
        System.out.println(entry.getKey());
        System.out.println(entry.getValue());
    }

    @Test
    public void testPromote() {
        assertThrows(
                ClientException.class,
                () -> {
                    Map.Entry<BlockId, Block> entry = client.promote(client.getTips()[0]);
                    System.out.println(entry.getKey());
                    System.out.println(entry.getValue());
                }
        );
    }

    @Test
    public void testPromoteUnchecked() throws ClientException {
        Map.Entry<BlockId, Block> entry = client.promoteUnchecked(client.getTips()[0]);
        System.out.println(entry.getKey());
        System.out.println(entry.getValue());
    }

}
