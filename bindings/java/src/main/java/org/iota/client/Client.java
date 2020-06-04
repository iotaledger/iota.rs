package org.iota.client;

import org.iota.client.models.NodeInfo;

public class Client {
    public Client(String url) {
        ClientNative.INSTANCE.iota_init(url);
    }

    public NodeInfo getNodeInfo() {
        return ClientNative.INSTANCE.iota_get_node_info();
    }
}
