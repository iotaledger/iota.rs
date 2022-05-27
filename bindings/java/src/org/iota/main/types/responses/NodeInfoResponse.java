package org.iota.main.types.responses;

import com.google.gson.JsonObject;

public class NodeInfoResponse extends ClientResponse {

    private String nodeUrl;
    private JsonObject nodeInfo;

    public NodeInfoResponse(BaseApiResponse response) {
        super(response);
        this.nodeUrl = response.getPayload().getAsJsonObject().get("url").getAsString();
        this.nodeInfo = response.getPayload().getAsJsonObject().get("node_info").getAsJsonObject();
    }

    public String getNodeUrl() {
        return nodeUrl;
    }

    public JsonObject getNodeInfo() {
        return nodeInfo;
    }

    @Override
    public String toString() {
        return "GetNodeInfoResponse{" +
                "response=" + response +
                '}';
    }
}
