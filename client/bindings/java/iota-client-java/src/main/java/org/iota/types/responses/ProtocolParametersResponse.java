// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.responses;

import com.google.gson.JsonObject;

public class ProtocolParametersResponse {

    private int version;
    private String networkName;
    private String bech32Hrp;
    private int minPowScore;
    private int belowMaxDepth;
    private JsonObject rentStructure;
    private String tokenSupply;

    public ProtocolParametersResponse(JsonObject response) {
        this.version = response.get("version").getAsInt();
        this.networkName = response.get("networkName").getAsString();
        this.bech32Hrp = response.get("bech32Hrp").getAsString();
        this.minPowScore = response.get("minPowScore").getAsInt();
        this.belowMaxDepth = response.get("belowMaxDepth").getAsInt();
        this.rentStructure = response.get("rentStructure").getAsJsonObject();
        this.tokenSupply = response.get("tokenSupply").getAsString();
    }

    public int getVersion() {
        return version;
    }

    public String getNetworkName() {
        return networkName;
    }

    public String getBech32Hrp() {
        return bech32Hrp;
    }

    public int getMinPowScore() {
        return minPowScore;
    }

    public int getBelowMaxDepth() {
        return belowMaxDepth;
    }

    public JsonObject getRentStructure() {
        return rentStructure;
    }

    public String getTokenSupply() {
        return tokenSupply;
    }
}
