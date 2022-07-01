// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.responses;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.ids.OutputId;

public class UtxoChangesResponse {

    private int index;
    private OutputId[] consumedOutputs;
    private OutputId[] createdOutputs;

    public UtxoChangesResponse(JsonObject response) {
        index = response.get("index").getAsInt();

        JsonArray consumedOutputs = response.getAsJsonArray("consumedOutputs");
        this.consumedOutputs = new OutputId[consumedOutputs.size()];
        for (int i = 0; i < consumedOutputs.size(); i++) {
            this.consumedOutputs[i] = new OutputId(consumedOutputs.get(i).getAsString());
        }

        JsonArray createdOutputs = response.getAsJsonArray("createdOutputs");
        this.createdOutputs = new OutputId[createdOutputs.size()];
        for (int i = 0; i < createdOutputs.size(); i++) {
            this.createdOutputs[i] = new OutputId(createdOutputs.get(i).getAsString());
        }
    }

    public int getIndex() {
        return index;
    }

    public OutputId[] getConsumedOutputs() {
        return consumedOutputs;
    }

    public OutputId[] getCreatedOutputs() {
        return createdOutputs;
    }

}
