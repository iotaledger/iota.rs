package org.iota;

public class Client {

    private ClientConfig config;

    public Client(ClientConfig config) {
        this.config = config;
    }

    // Node API

    public String getNodeHealth(String nodeUrl) {
        return RustApi.call(config, new ClientCommand(Command.CallClientMethod, "{ \"name\": \"GetNodeHealth\", \"data\": { \"url\": \"" + nodeUrl + "\" }}"));
    }

    public String getNodeInfo() {
        return RustApi.call(config, new ClientCommand(Command.CallClientMethod, "{ \"name\": \"GetInfo\" }"));
    }

}