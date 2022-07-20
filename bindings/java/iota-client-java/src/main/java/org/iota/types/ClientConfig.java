// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.JsonObject;

public class ClientConfig {

    private String primaryNode;
    private String primaryPoWNode;
    private String[] nodes;
    private String[] permanodes;
    private boolean nodeSyncEnabled = true;
    private NodeSyncInterval nodeSyncInterval = new NodeSyncInterval().withSecs(60);
    private boolean quorum;
    private int minQuorumSize = 3;
    private int quorumThreshold = 66;
    private String network;
    private String networkId;
    private String bech32HRP = "rms";
    private double minPoWScore = 4000.0;
    private boolean localPow = true;
    private boolean fallbackToLocalPow = true;
    private int tipsInterval = 15;
    private RentStructure rentStructure = new RentStructure(500, 10, 1);
    private ApiTimeout apiTimeout = new ApiTimeout().withSecs(15);
    private RemotePowTimeout remotePowTimeout = new RemotePowTimeout().withSecs(100);
    private boolean offline;
    private Integer powWorkerCount;

    public String getPrimaryNode() {
        return primaryNode;
    }

    public ClientConfig withPrimaryNode(String primaryNode) {
        this.primaryNode = primaryNode;
        return this;
    }

    public String getPrimaryPoWNode() {
        return primaryPoWNode;
    }

    public ClientConfig withPrimaryPoWNode(String primaryPoWNode) {
        this.primaryPoWNode = primaryPoWNode;
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

    public boolean isNodeSyncEnabled() {
        return nodeSyncEnabled;
    }

    public ClientConfig withNodeSyncEnabled(boolean nodeSyncEnabled) {
        this.nodeSyncEnabled = nodeSyncEnabled;
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

    public String getBech32HRP() {
        return bech32HRP;
    }

    public ClientConfig withBech32HRP(String bech32HRP) {
        this.bech32HRP = bech32HRP;
        return this;
    }

    public double getMinPoWScore() {
        return minPoWScore;
    }

    public ClientConfig withMinPoWScore(double minPoWScore) {
        this.minPoWScore = minPoWScore;
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

    public boolean isOffline() {
        return offline;
    }

    public ClientConfig withOffline(boolean offline) {
        this.offline = offline;
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
        o.addProperty("primaryNode", primaryNode);
        o.addProperty("primaryPoWNode", primaryPoWNode);
        o.add("nodes", JsonUtils.toJson(nodes));
        o.add("permanodes", JsonUtils.toJson(permanodes));
        o.addProperty("nodeSyncEnabled", nodeSyncEnabled);
        o.add("nodeSyncInterval", nodeSyncInterval != null ? nodeSyncInterval.getJson() : null);
        o.addProperty("quorum", quorum);
        o.addProperty("minQuorumSize", minQuorumSize);
        o.addProperty("quorumThreshold", quorumThreshold);
        o.addProperty("network", network);
        o.addProperty("networkId", networkId);
        o.addProperty("bech32HRP", bech32HRP);
        o.addProperty("minPoWScore", minPoWScore);
        o.addProperty("localPow", localPow);
        o.addProperty("fallbackToLocalPow", fallbackToLocalPow);
        o.addProperty("tipsInterval", tipsInterval);
        o.add("rentStructure", rentStructure != null ? rentStructure.getJson() : null);
        o.add("apiTimeout", apiTimeout != null ? apiTimeout.getJson() : null);
        o.add("remotePowTimeout", remotePowTimeout != null ? remotePowTimeout.getJson() : null);
        o.addProperty("offline", offline);
        o.addProperty("powWorkerCount", powWorkerCount);

        return o;
    }

}