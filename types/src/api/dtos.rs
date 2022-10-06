// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::block::payload::milestone::option::dto::ReceiptMilestoneOptionDto;

/// Describes a peer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PeerDto {
    pub id: String,
    #[serde(rename = "multiAddresses")]
    pub multi_addresses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub relation: RelationDto,
    pub connected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gossip: Option<GossipDto>,
}

/// Returns all information about the gossip stream with the peer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct GossipDto {
    pub heartbeat: HeartbeatDto,
    pub metrics: MetricsDto,
}

/// Describes the relation with the peer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RelationDto {
    #[serde(rename = "known")]
    Known,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "autopeered")]
    Autopeered,
}

/// Describes the heartbeat of a node.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct HeartbeatDto {
    #[serde(rename = "solidMilestoneIndex")]
    pub solid_milestone_index: u32,
    #[serde(rename = "prunedMilestoneIndex")]
    pub pruned_milestone_index: u32,
    #[serde(rename = "latestMilestoneIndex")]
    pub latest_milestone_index: u32,
    #[serde(rename = "connectedPeers")]
    pub connected_peers: u8,
    #[serde(rename = "syncedPeers")]
    pub synced_peers: u8,
}

/// Describes metrics of a gossip stream.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetricsDto {
    #[serde(rename = "newBlocks")]
    pub new_blocks: u64,
    #[serde(rename = "receivedBlocks")]
    pub received_blocks: u64,
    #[serde(rename = "knownBlocks")]
    pub known_blocks: u64,
    #[serde(rename = "receivedBlockRequests")]
    pub received_block_requests: u64,
    #[serde(rename = "receivedMilestoneRequests")]
    pub received_milestone_requests: u64,
    #[serde(rename = "receivedHeartbeats")]
    pub received_heartbeats: u64,
    #[serde(rename = "sentBlocks")]
    pub sent_blocks: u64,
    #[serde(rename = "sentBlockRequests")]
    pub sent_block_requests: u64,
    #[serde(rename = "sentMilestoneRequests")]
    pub sent_milestone_requests: u64,
    #[serde(rename = "sentHeartbeats")]
    pub sent_heartbeats: u64,
    #[serde(rename = "droppedPackets")]
    pub dropped_packets: u64,
}

/// Describes a receipt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReceiptDto {
    pub receipt: ReceiptMilestoneOptionDto,
    #[serde(rename = "milestoneIndex")]
    pub milestone_index: u32,
}

/// Describes the ledger inclusion state of a transaction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LedgerInclusionStateDto {
    #[serde(rename = "conflicting")]
    Conflicting,
    #[serde(rename = "included")]
    Included,
    #[serde(rename = "noTransaction")]
    NoTransaction,
}
