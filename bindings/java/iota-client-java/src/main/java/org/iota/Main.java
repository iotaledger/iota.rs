package org.iota;

import org.iota.types.ClientConfig;
import org.iota.types.ClientException;

public class Main {

    protected static final String DEFAULT_DEVNET_NODE_URL = "https://api.alphanet.iotaledger.net";
    protected static final String DEFAULT_DEVNET_FAUCET_URL = "https://faucet.alphanet.iotaledger.net";

    protected static ClientConfig config = new ClientConfig("{ \"nodes\": [\"" + DEFAULT_DEVNET_NODE_URL + "\" ], \"nodeSyncEnabled\": false}");


    public static void main(String[] args) throws ClientException {
        Client c = new Client(config);
        String hrp = c.getBech32Hrp();
    }
}
