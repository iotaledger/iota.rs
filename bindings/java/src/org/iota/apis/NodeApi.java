package org.iota.apis;

import org.iota.*;

public class NodeApi extends BaseApi {

    public NodeApi(ClientConfig config) {
        super(config);
    }

    public String getNodeHealth(String nodeUrl) {
        return RustApi.call(super.config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetNodeHealth\", \"data\": { \"url\": \"" + nodeUrl + "\" }}"));
    }

    public String getNodeInfo() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetInfo\" }"));
    }

}
