// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use getset::{CopyGetters, Getters};
use iota_client::bee_rest_api::types::dtos::{
    GossipDto as RustgossipDto, HeartbeatDto as RustheartbeatDto, MetricsDto as RustMetricsDto,
};

#[derive(Copy, Clone, Eq, PartialEq, Getters, CopyGetters, Debug)]
pub struct GossipDto {
    #[getset(get_copy = "pub")]
    pub heartbeat: HeartbeatDto,
    #[getset(get_copy = "pub")]
    pub metrics: MetricsDto,
}

impl core::fmt::Display for GossipDto {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "heartbeat={}, metrics={}", self.heartbeat, self.metrics)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Getters, CopyGetters, Debug)]
pub struct HeartbeatDto {
    #[getset(get_copy = "pub")]
    pub solid_milestone_index: u32,
    #[getset(get_copy = "pub")]
    pub pruned_milestone_index: u32,
    #[getset(get_copy = "pub")]
    pub latest_milestone_index: u32,
    #[getset(get_copy = "pub")]
    pub connected_neighbors: u8,
    #[getset(get_copy = "pub")]
    pub synced_neighbors: u8,
}

impl core::fmt::Display for HeartbeatDto {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "solid_milestone_index={}, latest_milestone_index={}, latest_milestone_index={}, connected_neighbors={}, synced_neighbors={}",
            self.solid_milestone_index, self.latest_milestone_index, self.latest_milestone_index, self.connected_neighbors, self.synced_neighbors
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Getters, CopyGetters, Debug)]
pub struct MetricsDto {
    #[getset(get_copy = "pub")]
    pub new_messages: u64,
    #[getset(get_copy = "pub")]
    pub received_messages: u64,
    #[getset(get_copy = "pub")]
    pub known_messages: u64,
    #[getset(get_copy = "pub")]
    pub received_message_requests: u64,
    #[getset(get_copy = "pub")]
    pub received_milestone_requests: u64,
    #[getset(get_copy = "pub")]
    pub received_heartbeats: u64,
    #[getset(get_copy = "pub")]
    pub sent_messages: u64,
    #[getset(get_copy = "pub")]
    pub sent_message_requests: u64,
    #[getset(get_copy = "pub")]
    pub sent_milestone_requests: u64,
    #[getset(get_copy = "pub")]
    pub sent_heartbeats: u64,
    #[getset(get_copy = "pub")]
    pub dropped_packets: u64,
}

impl core::fmt::Display for MetricsDto {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "new_messages={}, received_messages={}, known_messages={}, received_message_requests={}, received_milestone_requests={}, received_heartbeats={}, sent_messages={}, sent_message_requests={}, sent_message_requests={}, sent_heartbeats={}, dropped_packets={}",
            self.new_messages, self.received_messages, self.known_messages, self.received_message_requests, self.received_milestone_requests, self.received_heartbeats, self.sent_messages, self.sent_message_requests, self.sent_message_requests, self.sent_heartbeats, self.dropped_packets
        )
    }
}

impl From<RustgossipDto> for GossipDto {
    fn from(gossip: RustgossipDto) -> Self {
        Self {
            heartbeat: HeartbeatDto::from(gossip.heartbeat),
            metrics: MetricsDto::from(gossip.metrics),
        }
    }
}

impl From<RustheartbeatDto> for HeartbeatDto {
    fn from(heartbeat: RustheartbeatDto) -> Self {
        Self {
            solid_milestone_index: heartbeat.solid_milestone_index,
            pruned_milestone_index: heartbeat.pruned_milestone_index,
            latest_milestone_index: heartbeat.latest_milestone_index,
            connected_neighbors: heartbeat.connected_neighbors,
            synced_neighbors: heartbeat.synced_neighbors,
        }
    }
}

impl From<RustMetricsDto> for MetricsDto {
    fn from(metrics: RustMetricsDto) -> Self {
        Self {
            new_messages: metrics.new_messages,
            received_messages: metrics.received_messages,
            known_messages: metrics.known_messages,
            received_message_requests: metrics.received_message_requests,
            received_milestone_requests: metrics.received_milestone_requests,
            received_heartbeats: metrics.received_heartbeats,
            sent_messages: metrics.sent_messages,
            sent_message_requests: metrics.sent_message_requests,
            sent_milestone_requests: metrics.sent_milestone_requests,
            sent_heartbeats: metrics.sent_heartbeats,
            dropped_packets: metrics.dropped_packets,
        }
    }
}
