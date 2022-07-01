// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.responses;

import com.google.gson.JsonObject;

public class NodeInfoResponse {

    private String nodeUrl;
    private JsonObject nodeInfo;

    public NodeInfoResponse(JsonObject response) {
        this.nodeUrl = response.get("url").getAsString();
        this.nodeInfo = response.get("nodeInfo").getAsJsonObject();
    }

    public String getNodeUrl() {
        return nodeUrl;
    }

    public JsonObject getNodeInfo() {
        return nodeInfo;
    }

}
