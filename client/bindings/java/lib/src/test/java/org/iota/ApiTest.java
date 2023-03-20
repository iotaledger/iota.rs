// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.types.Block;
import org.iota.types.ClientConfig;
import org.iota.types.OutputMetadata;
import org.iota.types.TaggedDataPayload;
import org.iota.types.expections.ClientException;
import org.iota.types.expections.NoFundsReceivedFromFaucetException;
import org.iota.types.expections.InitializeClientException;
import org.iota.types.ids.OutputId;
import org.iota.types.ids.TransactionId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;
import org.iota.types.secret.SecretManager;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

public abstract class ApiTest {

    protected static final String DEFAULT_TESTNET_NODE_URL = "http://localhost:14265";
    protected Client client;
    protected ClientConfig config = new ClientConfig().withNodes(new String[] { DEFAULT_TESTNET_NODE_URL }).withIgnoreNodeHealth(false);

    @BeforeEach
    protected void setUp() throws InitializeClientException {
        client = new Client(config);
    }

    @AfterEach
    protected void tearDown() {
        client.destroyHandle();
    }

    protected Block setUpTaggedDataBlock() throws ClientException {
        return client.postBlockPayload(new TaggedDataPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }")).getValue();
    }

    protected TransactionId setUpTransactionId(String address) throws ClientException, InitializeClientException, NoFundsReceivedFromFaucetException {
        OutputMetadata metadata = client.getOutputMetadata(setupBasicOutput(address));
        TransactionId ret = new TransactionId(metadata.toJson().get("transactionId").getAsString());
        return ret;
    }

    protected OutputId setupBasicOutput(String address) throws ClientException, InitializeClientException, NoFundsReceivedFromFaucetException {
        client.requestTestFundsFromFaucet(address);
        return client.getBasicOutputIds(new NodeIndexerApi.QueryParams().withParam("address", address)).getItems()[0];
    }

    protected String generateAddress(String mnemonic) throws ClientException {
        SecretManager secretManager = new MnemonicSecretManager(mnemonic);
        String[] addresses = client.generateAddresses(secretManager, new GenerateAddressesOptions().withRange(new Range(0, 1)));
        return addresses[0];
    }

}