package org.iota.tests;

import org.iota.main.Client;
import org.iota.main.apis.UtilsApi;
import org.iota.main.types.Block;
import org.iota.main.types.BlockPayload;
import org.iota.main.types.ClientConfig;
import org.iota.main.types.ClientException;
import org.iota.main.types.responses.node_core_api.BlockResponse;
import org.iota.main.types.secret.GenerateAddressesOptions;
import org.iota.main.types.secret.GenerateBlockOptions;
import org.iota.main.types.secret.MnemonicSecretManager;
import org.junit.jupiter.api.BeforeEach;

public abstract class ApiTest {

    protected static final String DEFAULT_DEVNET_NODE_URL = "";
    protected static final String DEFAULT_DEVNET_FAUCET_URL = "";
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
        BlockResponse r = client.submitBlockPayload(new BlockPayload("{ \"type\": 5, \"tag\": \"0x68656c6c6f20776f726c64\", \"data\": \"0x5370616d6d696e6720646174612e0a436f756e743a203037323935320a54696d657374616d703a20323032312d30322d31315431303a32333a34392b30313a30300a54697073656c656374696f6e3a203934c2b573\" }"));
        return r.getBlock();
    }

    protected Block setUpTransactionBlock() throws ClientException {
        String address = client.generateAddresses(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new GenerateAddressesOptions().withRange(0, 1)).getAddresses()[0];
        requestFundsFromFaucet(address);
        try {
            Thread.sleep(1000 * 10);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        Block b = client.generateBlock(new MnemonicSecretManager(DEFAULT_DEVELOPMENT_MNEMONIC), new GenerateBlockOptions().withOutputHex(new GenerateBlockOptions.ClientBlockBuilderOutputAddress(client.bech32ToHex(address).getHexAddress(), "10000000"))).getBlock();
        try {
            Thread.sleep(1000 * 10);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        return b;
    }

    protected String setUpTransactionId() throws ClientException {
        Block b = setUpTransactionBlock();
        String transactionId = client.getTransactionId(new BlockPayload(b.getJson().get("payload").getAsJsonObject())).getTransactionId();
        return transactionId;
    }

    protected String setupOutputId() throws ClientException {
        return setUpTransactionId() + "0000";
    }

}
