// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.responses;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.ids.OutputId;

public class OutputIdsResponse {

    private int ledgerIndex;
    private String cursor;
    private OutputId[] items;

    public OutputIdsResponse(JsonObject response) {
        this.ledgerIndex = response.get("ledgerIndex").getAsInt();
        if (!response.get("cursor").isJsonNull()) { 
            this.cursor = response.get("cursor").getAsString();
        }

        JsonArray items = response.getAsJsonArray("items");
        this.items = new OutputId[items.size()];
        for (int i = 0; i < items.size(); i++) {
            this.items[i] = new OutputId(items.get(i).getAsString());
        }
    }

    public int getLedgerIndex() {
        return ledgerIndex;
    }

    public String getCursor() {
        return cursor;
    }

    public OutputId[] getItems() {
        return items;
    }

}