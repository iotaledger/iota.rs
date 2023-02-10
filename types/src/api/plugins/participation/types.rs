// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Participation types.

#![allow(missing_docs)]

extern crate alloc;
use std::{collections::HashMap, convert::TryInto, io::Read};

use getset::Getters;
use packable::PackableExt;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{api::plugins::participation::error::Error, impl_id, string_serde_impl};

/// Participation tag.
pub const PARTICIPATION_TAG: &str = "PARTICIPATE";

/// Possible participation event types.
#[derive(Debug, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[serde(untagged)]
#[repr(u8)]
pub enum ParticipationEventType {
    /// Voting event.
    Voting = 0,
    /// Staking event.
    Staking = 1,
}

/// Wrapper interface containing a participation event ID and the corresponding event data.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParticipationEventPayload {
    /// Voting payload.
    VotingEventPayload(VotingEventPayload),
    /// Staking payload.
    StakingEventPayload(StakingEventPayload),
}

/// Payload for a voting event.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct VotingEventPayload {
    #[serde(rename = "type")]
    kind: u32,
    questions: Vec<Question>,
}

/// Question for a voting event.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Question {
    text: String,
    answers: Vec<Answer>,
    additional_info: String,
}

/// Answer in a voting event.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct Answer {
    value: u8,
    text: String,
    additional_info: String,
}

/// Payload for a staking event.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct ParticipationEventStatus {
    milestone_index: u32,
    status: String,
    questions: Option<Vec<QuestionStatus>>,
    checksum: String,
}

/// Question status.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct QuestionStatus {
    answers: Vec<AnswerStatus>,
}

/// Answer status.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct AnswerStatus {
    value: u8,
    current: u64,
    accumulated: u64,
}

/// Staking rewards for an address.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressStakingStatus {
    /// Rewards for staking events.
    pub rewards: HashMap<String, StakingStatus>,
    /// MilestoneIndex is the milestone index the rewards were calculated for.
    pub milestone_index: u32,
}

/// Staking rewards for an address.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingStatus {
    /// Staked amount.
    pub amount: u64,
    /// Currency symbol.
    pub symbol: String,
    /// If the required minimum staking reward is reached.
    pub minimum_reached: bool,
}

/// Participation information.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participation {
    /// A staking or voting event id, hex encoded [u8; 32].
    pub event_id: ParticipationEventId,
    /// Answers for a voting event, can be empty.
    pub answers: Vec<u8>,
}

/// Participations information.
/// <https://github.com/iota-community/treasury/blob/main/specifications/hornet-participation-plugin.md#structure-of-the-participation>
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Participations {
    /// Multiple participations that happen at the same time.
    pub participations: Vec<Participation>,
}

impl Participations {
    /// Replace the answers if there is already a participation with the same event id or add the participation.
    pub fn add_or_replace(&mut self, participation: Participation) {
        if let Some(existing) = self
            .participations
            .iter_mut()
            .find(|p| p.event_id == participation.event_id)
        {
            existing.answers = participation.answers;
        } else {
            self.participations.push(participation);
        }
    }

    /// Remove participations with the provided event id.
    pub fn remove(&mut self, event_id: &ParticipationEventId) {
        self.participations.retain(|p| &p.event_id != event_id);
    }

    /// Serialize to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = vec![
            self.participations
                .len()
                .try_into()
                .map_err(|_| Error::InvalidParticipations)?,
        ];

        for participation in &self.participations {
            let event_id: Vec<u8> = participation.event_id.pack_to_vec();
            bytes.extend(event_id);
            bytes.push(
                participation
                    .answers
                    .len()
                    .try_into()
                    .map_err(|_| Error::InvalidParticipations)?,
            );
            for answer in &participation.answers {
                bytes.push(*answer);
            }
        }
        Ok(bytes)
    }

    /// Deserialize from bytes.
    pub fn from_bytes<R: Read + ?Sized>(bytes: &mut R) -> Result<Self, Error> {
        let mut participations = Vec::new();
        let mut participations_len = [0u8; 1];
        bytes.read_exact(&mut participations_len)?;

        for _ in 0..participations_len[0] {
            let mut event_id: [u8; 32] = [0u8; 32];
            bytes.read_exact(&mut event_id)?;

            let mut answers_len = [0u8; 1];
            bytes.read_exact(&mut answers_len)?;

            let mut answers = Vec::new();
            for _ in 0..answers_len[0] {
                let mut answer = [0u8; 1];
                bytes.read_exact(&mut answer)?;
                answers.push(answer[0]);
            }

            participations.push(Participation {
                event_id: ParticipationEventId::new(event_id),
                answers,
            });
        }

        Ok(Self { participations })
    }
}
