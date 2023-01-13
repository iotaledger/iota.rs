// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Participation types.

#![allow(missing_docs)]

/// Participation data.
pub mod participation;

extern crate alloc;
use std::collections::HashMap;

use getset::Getters;
use iota_types::{impl_id, string_serde_impl};
use serde::{Deserialize, Serialize};

/// Participation tag.
pub const PARTICIPATION_TAG: &str = "PARTICIPATE";

/// Possible participation event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParticipationEventType {
    /// Voting event.
    Voting,
    /// Staking event.
    Staking,
}

/// Wrapper interface containing a participation event ID and the corresponding event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipationEvent {
    /// The event id.
    pub id: ParticipationEventId,
    /// Information about a voting or staking event.
    pub data: ParticipationEventData,
}

impl_id!(pub ParticipationEventId, 32, "A participation event id.");
string_serde_impl!(ParticipationEventId);

/// Information about a voting or staking event.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct ParticipationEventData {
    name: String,
    milestone_index_commence: u32,
    milestone_index_start: u32,
    milestone_index_end: u32,
    payload: ParticipationEventPayload,
    additional_info: String,
}

/// Event payload types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParticipationEventPayload {
    /// Voting payload.
    VotingEventPayload(VotingEventPayload),
    /// Staking payload.
    StakingEventPayload(StakingEventPayload),
}

/// Payload for a voting event.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct VotingEventPayload {
    #[serde(rename = "type")]
    kind: u32,
    questions: Vec<Question>,
}

/// Question for a voting event.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Question {
    text: String,
    answers: Vec<Answer>,
    additional_info: String,
}

/// Answer in a voting event.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Answer {
    value: u8,
    text: String,
    additional_info: String,
}

/// Payload for a staking event.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct StakingEventPayload {
    #[serde(rename = "type")]
    kind: u32,
    text: String,
    symbol: String,
    numerator: u64,
    denominator: u64,
    required_minimum_rewards: u64,
    additional_info: String,
}

/// Event status.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct ParticipationEventStatus {
    milestone_index: u32,
    status: String,
    questions: Option<Vec<QuestionStatus>>,
    checksum: String,
}

/// Question status.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct QuestionStatus {
    answers: Vec<AnswerStatus>,
}

/// Answer status.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct AnswerStatus {
    value: u8,
    current: u64,
    accumulated: u64,
}

/// Staking rewards for an address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressStakingStatus {
    /// Rewards for staking events.
    pub rewards: HashMap<String, StakingStatus>,
    /// MilestoneIndex is the milestone index the rewards were calculated for.
    pub milestone_index: u32,
}

/// Staking rewards for an address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingStatus {
    /// Staked amount.
    pub amount: u64,
    /// Currency symbol.
    pub symbol: String,
    /// If the required minimum staking reward is reached.
    pub minimum_reached: bool,
}
