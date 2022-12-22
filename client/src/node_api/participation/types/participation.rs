// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;
use packable::error::{UnpackError, UnpackErrorExt};

use packable::packer::Packer;
use packable::prefix::{ VecPrefix};
use packable::unpacker::Unpacker;
use serde::{Deserialize, Serialize};
use crate::Error;

use crate::node_api::participation::types::EventId;

/// Participation information.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Participation {
    /// A staking or voting event id, hex encoded [u8; 32].
    pub event_id: EventId,
    /// Answers for a voting event, can be empty.
    pub answers: Vec<u8>,
}

impl Packable for Participation {
    type UnpackError =  Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.event_id.pack(packer)?;
        let vec_prefix: VecPrefix<u8, u8> = VecPrefix::try_from(self.answers.clone()).unwrap();
        vec_prefix.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(unpacker: &mut U, visitor: &Self::UnpackVisitor) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let event_id = EventId::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let vec_prefix: VecPrefix<u8, u8> = VecPrefix::<u8, u8>::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;

        Ok(Self {
            event_id,
            answers: vec_prefix.to_vec(),
        })
    }
}

/// Participations information.
/// https://github.com/iota-community/treasury/blob/main/specifications/hornet-participation-plugin.md#structure-of-the-participation
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Participations {
    /// Multiple participations that happen at the same time.
    pub participations: Vec<Participation>,
}

impl Packable for Participations {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        let vec_prefix: VecPrefix<Participation, u8> = VecPrefix::try_from(self.participations.clone()).unwrap();
        vec_prefix.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(unpacker: &mut U, visitor: &Self::UnpackVisitor) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let vec_prefix: VecPrefix<Participation, u8> = VecPrefix::<Participation, u8>::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;

        Ok(Self {
            participations: vec_prefix.to_vec(),
        })
    }
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
    pub fn remove(&mut self, event_id: &EventId) {
        self.participations.retain(|p| &p.event_id != event_id);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use packable::PackableExt;

    use super::{Participation, Participations};
    use crate::node_api::participation::types::EventId;

    #[test]
    fn testx() {
        assert_eq!(true,false)
    }

    #[test]
    fn serialize_deserialize_packable() {
        let participations = Participations {
            participations: vec![
                Participation {
                    event_id: EventId::from_str("0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111")
                        .unwrap(),
                    answers: vec![0, 1],
                },
                Participation {
                    event_id: EventId::from_str("0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f")
                        .unwrap(),
                    answers: vec![],
                },
            ],
        };

        let participation_bytes = participations.pack_to_vec();
        let mut slice: &[u8] = &participation_bytes;
        let deserialized_participations: Participations = Participations::unpack_verified(&mut slice, &()).unwrap();

        assert_eq!(participations, deserialized_participations);
    }

    #[test]
    fn mutate() {
        let mut participations = Participations {
            participations: vec![
                Participation {
                    event_id: EventId::from_str("0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111")
                        .unwrap(),
                    answers: vec![0, 1],
                },
                Participation {
                    event_id: EventId::from_str("0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111")
                        .unwrap(),
                    answers: vec![0, 1],
                },
                Participation {
                    event_id: EventId::from_str("0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f")
                        .unwrap(),
                    answers: vec![],
                },
            ],
        };

        participations
            .remove(&EventId::from_str("0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111").unwrap());

        // replace
        participations.add_or_replace(Participation {
            event_id: EventId::from_str("0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f").unwrap(),
            answers: vec![1],
        });

        // add
        participations.add_or_replace(Participation {
            event_id: EventId::from_str("0x80f57f6368933b61af9b3d8e1b152cf5d23bf4537f6362778b0a7302a7000d48").unwrap(),
            answers: vec![1, 2],
        });

        assert_eq!(
            participations,
            Participations {
                participations: vec![
                    Participation {
                        event_id: EventId::from_str(
                            "0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f"
                        )
                        .unwrap(),
                        answers: vec![1],
                    },
                    Participation {
                        event_id: EventId::from_str(
                            "0x80f57f6368933b61af9b3d8e1b152cf5d23bf4537f6362778b0a7302a7000d48"
                        )
                        .unwrap(),
                        answers: vec![1, 2],
                    }
                ],
            }
        );
    }
}
