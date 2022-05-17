package org.iota;

import org.iota.apis.NodeApi;

public class Client {

    private NodeApi nodeApi;

    public Client(ClientConfig config) {
        nodeApi = new NodeApi(config);
    }

    public String getNodeHealth(String nodeUrl) {
        return nodeApi.getNodeHealth(nodeUrl);
    }

    public String getNodeInfo() {
        return nodeApi.getNodeInfo();
    }

}