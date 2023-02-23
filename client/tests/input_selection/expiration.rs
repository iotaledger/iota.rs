// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, str::FromStr};

use iota_client::{
    api::input_selection::{Error, InputSelection},
    block::{
        output::{AliasId, NftId},
        protocol::protocol_parameters,
    },
};

use crate::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Alias, Basic, Nft},
    ALIAS_ID_1, BECH32_ADDRESS_ALIAS_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2,
    NFT_ID_1,
};

#[test]
fn one_output_expiration_not_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 200)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn expiration_equal_timestamp() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 200)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(200)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_output_expiration_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_expiration_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 200)),
            None,
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 50)),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_unexpired_one_missing() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 200)),
            None,
        ),
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_two_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_0, 100)),
            None,
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 100)),
            None,
        ),
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .timestamp(200)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_two_expired_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 100)),
            None,
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 100)),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        4_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2]),
        protocol_parameters,
    )
    .timestamp(200)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_and_timelock() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 1_000_000)),
        Some(50),
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_and_timelock_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        Some(50),
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sender_in_expiration() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 50)),
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[2]));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sender_in_expiration_already_selected() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .timestamp(100)
    .required_inputs(HashSet::from_iter([*inputs[0].output_id()]))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn remainder_in_expiration() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ALIAS_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_1),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_1,
                None
            ));
        }
    });
}

#[test]
fn expiration_expired_non_ed25519_in_address_unlock_condition() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ALIAS_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_only_alias_addresses() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ALIAS_1,
            None,
            None,
            None,
            None,
            Some((BECH32_ADDRESS_ALIAS_1, 50)),
            None,
        ),
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);

    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn one_nft_output_expiration_unexpired() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![Nft(
        2_000_000,
        nft_id_1,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 150)),
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        2_000_000,
        nft_id_1,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_1]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_nft_output_expiration_expired() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![Nft(
        2_000_000,
        nft_id_1,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_0, 50)),
        None,
    )]);
    let outputs = build_outputs(vec![Nft(
        2_000_000,
        nft_id_1,
        BECH32_ADDRESS_ED25519_1,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .timestamp(100)
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
