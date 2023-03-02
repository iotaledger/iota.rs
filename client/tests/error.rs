// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::Error as IsaError, block::Error as BlockError, Error};

#[test]
fn stringified_error() {
    let error = Error::InvalidAmount("0".into());
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"invalidAmount\",\"error\":\"invalid amount in API response: 0\"}"
    );

    let error = Error::TimeNotSynced {
        current_time: 0,
        milestone_timestamp: 10000,
    };
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"timeNotSynced\",\"error\":\"local time 0 doesn't match the time of the latest milestone timestamp: 10000\"}"
    );

    let error = Error::InvalidBIP32ChainData;
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"invalidBIP32ChainData\",\"error\":\"invalid BIP32 chain data\"}"
    );

    let error = Error::InputSelection(IsaError::InsufficientAmount {
        found: 0,
        required: 100,
    });
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"inputSelection\",\"error\":\"insufficient amount: found 0, required 100\"}"
    );

    let error = Error::InputSelection(IsaError::Block(BlockError::InvalidAddress));
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"inputSelection\",\"error\":\"invalid address provided\"}"
    );
}
