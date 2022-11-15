// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class ClientConfig {

    private String primaryNode;
    private String primaryPowNode;
    private String[] nodes;
    private String[] permanodes;
    private Boolean ignoreNodeHealth;
    private NodeSyncInterval nodeSyncInterval;
    private Boolean quorum;
    private Integer minQuorumSize;
    private Integer quorumThreshold;
    private String network;
    private String networkId;
    private String bech32Hrp;
    private Integer minPowScore;
    private Boolean localPow;
    private Boolean fallbackToLocalPow;
    private Integer tipsInterval;
    private RentStructure rentStructure;
    private ApiTimeout apiTimeout;
    private RemotePowTimeout remotePowTimeout;
    private Integer powWorkerCount;

    public String getPrimaryNode() {
        return primaryNode;
    }

    public ClientConfig withPrimaryNode(String primaryNode) {
        this.primaryNode = primaryNode;
        return this;
    }

    public String getPrimaryPowNode() {
        return primaryPowNode;
    }

    public ClientConfig withPrimaryPowNode(String primaryPowNode) {
        this.primaryPowNode = primaryPowNode;
        return this;
    }

    public String[] getNodes() {
        return nodes;
    }

    public ClientConfig withNodes(String[] nodes) {
        this.nodes = nodes;
        return this;
    }

    public String[] getPermanodes() {
        return permanodes;
    }

    public ClientConfig withPermanodes(String[] permanodes) {
        this.permanodes = permanodes;
        return this;
    }

    public boolean isIgnoreNodeHealth() {
        return ignoreNodeHealth;
    }

    public ClientConfig withIgnoreNodeHealth(boolean ignoreNodeHealth) {
        this.ignoreNodeHealth = ignoreNodeHealth;
        return this;
    }

    public NodeSyncInterval getNodeSyncInterval() {
        return nodeSyncInterval;
    }

    public ClientConfig withNodeSyncInterval(NodeSyncInterval nodeSyncInterval) {
        this.nodeSyncInterval = nodeSyncInterval;
        return this;
    }

    public boolean isQuorum() {
        return quorum;
    }

    public ClientConfig withQuorum(boolean quorum) {
        this.quorum = quorum;
        return this;
    }

    public int getMinQuorumSize() {
        return minQuorumSize;
    }

    public ClientConfig withMinQuorumSize(int minQuorumSize) {
        this.minQuorumSize = minQuorumSize;
        return this;
    }

    public int getQuorumThreshold() {
        return quorumThreshold;
    }

    public ClientConfig withQuorumThreshold(int quorumThreshold) {
        this.quorumThreshold = quorumThreshold;
        return this;
    }

    public String getNetwork() {
        return network;
    }

    public ClientConfig withNetwork(String network) {
        this.network = network;
        return this;
    }

    public String getNetworkId() {
        return networkId;
    }

    public ClientConfig withNetworkId(String networkId) {
        this.networkId = networkId;
        return this;
    }

    public String getBech32Hrp() {
        return bech32Hrp;
    }

    public ClientConfig withBech32Hrp(String bech32Hrp) {
        this.bech32Hrp = bech32Hrp;
        return this;
    }

    public int getMinPowScore() {
        return minPowScore;
    }

    public ClientConfig withMinPowScore(int minPowScore) {
        this.minPowScore = minPowScore;
        return this;
    }

    public boolean isLocalPow() {
        return localPow;
    }

    public ClientConfig withLocalPow(boolean localPow) {
        this.localPow = localPow;
        return this;
    }

    public boolean isFallbackToLocalPow() {
        return fallbackToLocalPow;
    }

    public ClientConfig withFallbackToLocalPow(boolean fallbackToLocalPow) {
        this.fallbackToLocalPow = fallbackToLocalPow;
        return this;
    }

    public int getTipsInterval() {
        return tipsInterval;
    }

    public ClientConfig withTipsInterval(int tipsInterval) {
        this.tipsInterval = tipsInterval;
        return this;
    }

    public RentStructure getRentStructure() {
        return rentStructure;
    }

    public ClientConfig withRentStructure(RentStructure rentStructure) {
        this.rentStructure = rentStructure;
        return this;
    }

    public Integer getPowWorkerCount() {
        return powWorkerCount;
    }

    public ClientConfig withPowWorkerCount(Integer powWorkerCount) {
        this.powWorkerCount = powWorkerCount;
        return this;
    }

    static class NodeSyncInterval {
        private int secs;
        private int nanos;

        public NodeSyncInterval withSecs(int secs) {
            this.secs = secs;
            return this;
        }

        public NodeSyncInterval withNanos(int nanos) {
            this.nanos = nanos;
            return this;
        }

        public JsonObject getJson() {
            JsonObject o = new JsonObject();
            o.addProperty("secs", secs);
            o.addProperty("nanos", nanos);
            return o;
        }
    }

    static class RentStructure {
        private int vByteCost;
        private int vByteFactorKey;
        private int vByteFactorData;

        public RentStructure(int vByteCost, int vByteFactorKey, int vByteFactorData) {
            this.vByteCost = vByteCost;
            this.vByteFactorKey = vByteFactorKey;
            this.vByteFactorData = vByteFactorData;
        }

        public JsonObject getJson() {
            JsonObject o = new JsonObject();
            o.addProperty("vByteCost", vByteCost);
            o.addProperty("vByteFactorKey", vByteFactorKey);
            o.addProperty("vByteFactorData", vByteFactorData);
            return o;
        }
    }

    static class ApiTimeout {
        private int secs;
        private int nanos;

        public ApiTimeout withSecs(int secs) {
            this.secs = secs;
            return this;
        }

        public ApiTimeout withNanos(int nanos) {
            this.nanos = nanos;
            return this;
        }

        public JsonObject getJson() {
            JsonObject o = new JsonObject();
            o.addProperty("secs", secs);
            o.addProperty("nanos", nanos);
            return o;
        }
    }

    static class RemotePowTimeout {
        private int secs;
        private int nanos;

        public RemotePowTimeout withSecs(int secs) {
            this.secs = secs;
            return this;
        }

        public RemotePowTimeout withNanos(int nanos) {
            this.nanos = nanos;
            return this;
        }

        public JsonObject getJson() {
            JsonObject o = new JsonObject();
            o.addProperty("secs", secs);
            o.addProperty("nanos", nanos);
            return o;
        }
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        if (primaryNode != null)
            o.addProperty("primaryNode", primaryNode);
        if (primaryPowNode != null)
            o.addProperty("primaryPowNode", primaryPowNode);
        if (nodes != null)
            o.add("nodes", JsonUtils.toJson(nodes));
        if (permanodes != null)
            o.add("permanodes", JsonUtils.toJson(permanodes));
        if (ignoreNodeHealth != null)
            o.addProperty("ignoreNodeHealth", ignoreNodeHealth);
        if (nodeSyncInterval != null)
            o.add("nodeSyncInterval", nodeSyncInterval.getJson());
        if (quorum != null)
            o.addProperty("quorum", quorum);
        if (minQuorumSize != null)
            o.addProperty("minQuorumSize", minQuorumSize);
        if (quorumThreshold != null)
            o.addProperty("quorumThreshold", quorumThreshold);
        if (network != null)
            o.addProperty("network", network);
        if (networkId != null)
            o.addProperty("networkId", networkId);
        if (bech32Hrp != null)
            o.addProperty("bech32Hrp", bech32Hrp);
        if (minPowScore != null)
            o.addProperty("minPowScore", minPowScore);
        if (localPow != null)
            o.addProperty("localPow", localPow);
        if (fallbackToLocalPow != null)
            o.addProperty("fallbackToLocalPow", fallbackToLocalPow);
        if (tipsInterval != null)
            o.addProperty("tipsInterval", tipsInterval);
        if (rentStructure != null)
            o.add("rentStructure", rentStructure.getJson());
        if (apiTimeout != null)
            o.add("apiTimeout", apiTimeout.getJson());
        if (powWorkerCount != null)
            o.addProperty("powWorkerCount", powWorkerCount);

        return o;
    }

}