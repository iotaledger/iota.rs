package org.iota.main.types.responses;

import com.google.gson.JsonObject;

public class GetNodeInfoResponse implements ClientResponse {

    private JsonObject nodeInfo;

    public GetNodeInfoResponse(JsonObject nodeInfo) {
        this.nodeInfo = nodeInfo;
    }

    public JsonObject getNodeInfo() {
        return nodeInfo;
    }

    @Override
    public String toString() {
        return nodeInfo.toString();
    }
}
