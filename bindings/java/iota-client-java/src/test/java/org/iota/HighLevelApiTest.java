// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.types.Block;
import org.iota.types.ClientException;
import org.iota.types.UtxoInput;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.OutputId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;
import org.iota.types.secret.SecretManager;
import org.junit.jupiter.api.Test;

import java.util.LinkedHashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertThrows;

public class HighLevelApiTest extends ApiTest {

    @Test
    public void testGetOutputs() throws ClientException {
        OutputId[] outputs = client.getBasicOutputIds(new NodeIndexerApi.QueryParams());
        for (Map.Entry e : client.getOutputs(outputs)) {
            System.out.println(e.getKey());
        }
    }

    @Test
    public void testTryGetOutputs() throws ClientException {
        OutputId[] outputs = client.getBasicOutputIds(new NodeIndexerApi.QueryParams());
        for (Map.Entry e : client.tryGetOutputs(outputs)) {
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
        LinkedHashMap<BlockId, Block> ret = client.retryUntilIncluded(client.getTips()[0], 1, 10);
    }

    @Test
    public void testConsolidateFunds() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String address = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(0, 1))[0];
        requestFundsFromFaucet(address);
        String consolidatedAddress = client.consolidateFunds(secretManager, 0, new Range(0, 5));
        System.out.println(consolidatedAddress);
    }

    @Test
    public void testFindInputs() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(0, 5));
        requestFundsFromFaucet(addresses[0]);
        UtxoInput[] inputs = client.findInputs(addresses, 1000);
        for (UtxoInput id : inputs)
            System.out.println(id);
    }

    @Test
    public void testFindOutputs() throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(0, 5));
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
