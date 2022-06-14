package org.iota.tests;

import org.iota.main.Client;
import org.iota.main.apis.UtilsApi;
import org.iota.main.types.*;
import org.iota.main.types.secret.GenerateAddressesOptions;
import org.iota.main.types.secret.GenerateBlockOptions;
import org.iota.main.types.secret.MnemonicSecretManager;
import org.junit.jupiter.api.BeforeEach;

public abstract class ApiTest {

    protected static final String DEFAULT_DEVNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    protected static final String DEFAULT_DEVNET_FAUCET_URL = "https://faucet.alphanet.iotaledger.net";
    protected static final String DEFAULT_DEVELOPMENT_MNEMONIC = "hidden enroll proud copper decide negative orient asset speed work dolphin atom unhappy game cannon scheme glow kid ring core name still twist actor";

    protected Client client;
    protected ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_DEVNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");

    @BeforeEach
    protected void setUp() {
        client = new Client(config);
    }

    protected void requestFundsFromFaucet(String address) throws ClientException {
        new UtilsApi(config).requestFundsFromFaucet(DEFAULT_DEVNET_FAUCET_URL, address);
    }

    protected Block setUpTaggedDataBlock() throws ClientException {
        return client.submitBlockPayload(new TransactionPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }"));
    }

    protected Block setUpTransactionBlock() throws ClientException {
        String address = client.generateAddresses(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new GenerateAddressesOptions().withRange(0, 1))[0];
        requestFundsFromFaucet(address);
        try {
            Thread.sleep(1000 * 10);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        Block b = client.generateBlock(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new GenerateBlockOptions().withOutputHex(new GenerateBlockOptions.ClientBlockBuilderOutputAddress(client.bech32ToHex(address), "10000000")));
        try {
            Thread.sleep(1000 * 10);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        return b;
    }

    protected TransactionId setUpTransactionId() throws ClientException {
        Block b = setUpTransactionBlock();
        TransactionId transactionId = client.getTransactionId(new TransactionPayload(b.getJson().get("payload").getAsJsonObject()));
        return transactionId;
    }

    protected OutputId setupOutputId() throws ClientException {
        return new OutputId(setUpTransactionId() + "0000");
    }

}
