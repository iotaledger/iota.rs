package org.iota;

import org.iota.apis.NodeApi;

public class Client {

    private NodeApi nodeApi;

    public Client(ClientConfig config) {
        nodeApi = new NodeApi(config);
    }

    public String getHealth(String nodeUrl) {
        return nodeApi.getHealth(nodeUrl);
    }

    public String getNodeInfo() {
        return nodeApi.getNodeInfo();
    }

    public String getTips() {
        return nodeApi.getTips();
    }

    public String getMessage(String messageId) {
        return nodeApi.getMessage(messageId);
    }

    public String getMessageMetadata(String messageId) {
        return nodeApi.getMessageMetadata(messageId);
    }

}