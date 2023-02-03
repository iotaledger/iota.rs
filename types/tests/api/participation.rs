// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_types::api::plugins::participation::types::{Participation, ParticipationEventId, Participations};

#[test]
fn serialize_deserialize() {
    let participations = Participations {
        participations: vec![
            Participation {
                event_id: ParticipationEventId::from_str(
                    "0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111",
                )
                .unwrap(),
                answers: vec![0, 1],
            },
            Participation {
                event_id: ParticipationEventId::from_str(
                    "0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f",
                )
                .unwrap(),
                answers: vec![],
            },
        ],
    };
    let participation_bytes = participations.to_bytes().unwrap();
    let mut slice: &[u8] = &participation_bytes;
    let deserialized_participations: Participations = Participations::from_bytes(&mut slice).unwrap();

    assert_eq!(participations, deserialized_participations);
}

#[test]
fn mutate() {
    let mut participations = Participations {
        participations: vec![
            Participation {
                event_id: ParticipationEventId::from_str(
                    "0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111",
                )
                .unwrap(),
                answers: vec![0, 1],
            },
            Participation {
                event_id: ParticipationEventId::from_str(
                    "0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111",
                )
                .unwrap(),
                answers: vec![0, 1],
            },
            Participation {
                event_id: ParticipationEventId::from_str(
                    "0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f",
                )
                .unwrap(),
                answers: vec![],
            },
        ],
    };

    participations.remove(
        &ParticipationEventId::from_str("0x09c2338f3acd51e626cc074d1abcb12d747076ddfccd5215d8f2f21af1aac111").unwrap(),
    );

    // replace
    participations.add_or_replace(Participation {
        event_id: ParticipationEventId::from_str("0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f")
            .unwrap(),
        answers: vec![1],
    });

    // add
    participations.add_or_replace(Participation {
        event_id: ParticipationEventId::from_str("0x80f57f6368933b61af9b3d8e1b152cf5d23bf4537f6362778b0a7302a7000d48")
            .unwrap(),
        answers: vec![1, 2],
    });

    assert_eq!(
        participations,
        Participations {
            participations: vec![
                Participation {
                    event_id: ParticipationEventId::from_str(
                        "0x0207c34ae298b90d85455eee718037ad84a46bd784cbe5fdd8c534cc955efa1f"
                    )
                    .unwrap(),
                    answers: vec![1],
                },
                Participation {
                    event_id: ParticipationEventId::from_str(
                        "0x80f57f6368933b61af9b3d8e1b152cf5d23bf4537f6362778b0a7302a7000d48"
                    )
                    .unwrap(),
                    answers: vec![1, 2],
                }
            ],
        }
    );
}
