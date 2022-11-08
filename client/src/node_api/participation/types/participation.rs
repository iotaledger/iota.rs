// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Participation types

use std::{convert::TryInto, io::Read};

use serde::{Deserialize, Serialize};

/// Participation information.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Participation {
    /// A staking or voting event id, hex encoded [u8; 32]
    #[serde(rename = "eventId")]
    pub event_id: String,
    /// Answers for a voting event, can be empty
    pub answers: Vec<u8>,
}

/// Participation information.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Participations {
    /// Multiple participations that happen at the same time
    pub participations: Vec<Participation>,
}

impl Participations {
    // https://github.com/alexsporn/treasury/blob/main/specifications/chrysalis-referendum-rfc.md#structure-of-the-participation
    /// Serialize to bytes
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = vec![
            self.participations
                .len()
                .try_into()
                .map_err(|_| crate::Error::InvalidParticipations)?,
        ];

        for participation in &self.participations {
            let event_id: Vec<u8> = prefix_hex::decode(&participation.event_id)?;
            bytes.extend(event_id);
            bytes.push(
                participation
                    .answers
                    .len()
                    .try_into()
                    .map_err(|_| crate::Error::InvalidParticipations)?,
            );
            for answer in &participation.answers {
                bytes.push(*answer);
            }
        }
        Ok(bytes)
    }
    /// Deserialize from bytes
    pub fn from_bytes<R: Read + ?Sized>(bytes: &mut R) -> crate::Result<Participations> {
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
                event_id: prefix_hex::encode(event_id),
                answers,
            });
        }

        Ok(Participations { participations })
    }
}

#[cfg(test)]
mod tests {
    use super::{Participation, Participations};

    #[test]
    fn serialize_deserialize() {
        let participations = Participations {
            participations: vec![
                Participation {
                    event_id: "0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111".to_string(),
                    answers: vec![0, 1],
                },
                Participation {
                    event_id: "0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f".to_string(),
                    answers: vec![],
                },
            ],
        };
        let participation_bytes = participations.to_bytes().unwrap();
        let mut slice: &[u8] = &participation_bytes;
        let deserialized_participations: Participations = Participations::from_bytes(&mut slice).unwrap();

        assert_eq!(participations, deserialized_participations);
    }
}
