package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonObject;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

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

}
