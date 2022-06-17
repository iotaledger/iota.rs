package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.types.*;
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
        for (Output o : client.getOutputs(outputs)) {
            System.out.println(o);
        }
    }

    @Test
    public void testTryGetOutputs() throws ClientException {
        OutputId[] outputs = client.getBasicOutputIds(new NodeIndexerApi.QueryParams());
        for (Output o : client.tryGetOutputs(outputs)) {
            System.out.println(o);
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
        String consolidatedAddress = client.consolidateFunds(secretManager, 0, new Range(0,5));
        System.out.println(consolidatedAddress);
    }

}
