// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{convert::TryInto, io::Read};

use packable::PackableExt;
use serde::{Deserialize, Serialize};

use super::{super::Error, ParticipationEventId};

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
/// https://github.com/iota-community/treasury/blob/main/specifications/hornet-participation-plugin.md#structure-of-the-participation
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
