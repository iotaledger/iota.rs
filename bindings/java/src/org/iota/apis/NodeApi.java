package org.iota.apis;

import org.iota.*;

public class NodeApi extends BaseApi {

    public NodeApi(ClientConfig config) {
        super(config);
    }

    public String getHealth(String nodeUrl) {
        return RustApi.call(super.config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetNodeHealth\", \"data\": { \"url\": \"" + nodeUrl + "\" }}"));
    }

    public String getNodeInfo() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetInfo\" }"));
    }

    public String getTips() {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetTips\" }"));
    }

    public String getMessage(String messageId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMessageData\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }

    public String getMessageMetadata(String messageId) {
        return RustApi.call(config, new ClientCommand(ClientCommandType.CallClientMethod, "{ \"name\": \"GetMessageMetadata\", \"data\": { \"messageId\": \"" + messageId + "\" }}"));
    }



}
