// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.apis.UtilsApi;
import org.iota.types.*;
import org.iota.types.ids.OutputId;
import org.iota.types.ids.TransactionId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;
import org.iota.types.secret.SecretManager;
import org.junit.jupiter.api.BeforeEach;

public abstract class ApiTest {

    protected static final String DEFAULT_TESTNET_NODE_URL = "https://api.testnet.shimmer.network";
    protected static final String DEFAULT_TESTNET_FAUCET_URL = "https://faucet.testnet.shimmer.network/api/enqueue";
    protected static final String DEFAULT_DEVELOPMENT_MNEMONIC = "hidden enroll proud copper decide negative orient asset speed work dolphin atom unhappy game cannon scheme glow kid ring core name still twist actor";

    protected Client client;
    protected ClientConfig config = new ClientConfig().withNodes(new String[] { DEFAULT_TESTNET_NODE_URL }).withIgnoreNodeHealth(false);

    @BeforeEach
    protected void setUp() {
        client = new Client(config);
    }

    protected void requestFundsFromFaucet(String address) throws ClientException {
        OutputId[] outputIds = client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address));

        if(outputIds.length == 0) {
            new UtilsApi(config).requestFundsFromFaucet(DEFAULT_TESTNET_FAUCET_URL, address);
            try {
                Thread.sleep(1000 * 15);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }

    protected Block setUpTaggedDataBlock() throws ClientException {
        return client.postBlockPayload(new TaggedDataPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }")).getValue();
    }

    protected TransactionId setUpTransactionId(String address) throws ClientException {
        OutputMetadata metadata = client.getOutputMetadata(setupOutputId(address));
        TransactionId ret = new TransactionId(metadata.toJson().get("transactionId").getAsString());
        client.getIncludedBlock(ret);
        return ret;
    }

    protected OutputId setupOutputId(String address) throws ClientException {
        requestFundsFromFaucet(address);
        return client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address))[0];
    }

    protected String generateAddress(String mnemonic) throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC);
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)));
        return addresses[0];
    }

}