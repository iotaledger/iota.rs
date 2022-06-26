// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.apis.NodeIndexerApi;
import org.iota.apis.UtilsApi;
import org.iota.types.*;
import org.iota.types.ids.OutputId;
import org.iota.types.ids.TransactionId;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.GenerateBlockOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.junit.jupiter.api.BeforeEach;

public abstract class ApiTest {

    protected static final String DEFAULT_DEVNET_NODE_URL = "http://localhost:14265";
    protected static final String DEFAULT_DEVNET_FAUCET_URL = "http://localhost:14265";
    protected static final String DEFAULT_DEVELOPMENT_MNEMONIC = "hidden enroll proud copper decide negative orient asset speed work dolphin atom unhappy game cannon scheme glow kid ring core name still twist actor";

    protected Client client;
    protected ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_DEVNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    @BeforeEach
    protected void setUp() {
        client = new Client(config);
    }

    protected void requestFundsFromFaucet(String address) throws ClientException {
        new UtilsApi(config).requestFundsFromFaucet(DEFAULT_DEVNET_FAUCET_URL, address);
        try {
            Thread.sleep(1000 * 25);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }

    protected Block setUpTaggedDataBlock() throws ClientException {
        return client.submitBlockPayload(new TransactionPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }"));
    }

    protected TransactionId setUpTransactionId() throws ClientException {
        OutputId outputId = client.getBasicOutputIds(new NodeIndexerApi.QueryParams())[0];
        OutputMetadata metadata = client.getOutputMetadata(outputId);
        return new TransactionId(metadata.getJson().get("transactionId").getAsString());
    }

    protected OutputId setupOutputId() throws ClientException {
        return client.getBasicOutputIds(new NodeIndexerApi.QueryParams())[0];
    }

}
