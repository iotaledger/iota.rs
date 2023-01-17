// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        address::Address,
        output::{AliasId, NftId, SimpleTokenScheme},
        protocol::protocol_parameters,
    },
    Error,
};
use primitive_types::U256;

use crate::input_selection::{
    build_inputs, build_outputs,
    Build::{Alias, Basic, Foundry, Nft},
    ALIAS_ID_0, ALIAS_ID_1, BECH32_ADDRESS_ED25519_0, NFT_ID_0, NFT_ID_1,
};

#[test]
fn burn_alias_present() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .burn(Burn::new().add_alias(alias_id_1))
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_alias_id_zero() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_inputs(vec![
        Nft(1_000_000, nft_id_0, BECH32_ADDRESS_ED25519_0, None, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);
    let nft_id = NftId::from(inputs[0].output_id());

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .burn(Burn::new().add_nft(nft_id))
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_alias_absent() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .burn(Burn::new().add_alias(alias_id_1))
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, false))) if alias_id == alias_id_1
    ));
}

#[test]
fn burn_nft_present() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Nft(1_000_000, nft_id_1, BECH32_ADDRESS_ED25519_0, None, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .burn(Burn::new().add_nft(nft_id_1))
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_nft_id_zero() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_0, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);
    let alias_id = AliasId::from(inputs[0].output_id());

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .burn(Burn::new().add_alias(alias_id))
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[0]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn burn_nft_absent() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .burn(Burn::new().add_nft(nft_id_1))
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Nft(nft_id))) if nft_id == nft_id_1
    ));
}

#[test]
fn burn_foundry_present() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Foundry(
            1_000_000,
            alias_id_1,
            SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
            None,
        ),
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(500_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .burn(Burn::new().add_foundry(inputs[0].output.as_foundry().id()))
        .select()
        .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[0]));
    assert!(selected.inputs.contains(&inputs[1]));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            if output.is_basic() {
                assert_eq!(output.amount(), 1_500_000);
                assert_eq!(output.as_basic().native_tokens().len(), 0);
                assert_eq!(output.as_basic().unlock_conditions().len(), 1);
                assert_eq!(output.as_basic().features().len(), 0);
                assert_eq!(
                    *output.as_basic().address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
            } else if output.is_alias() {
                assert_eq!(output.amount(), 1_000_000);
                assert_eq!(output.as_alias().native_tokens().len(), 0);
                assert_eq!(*output.as_alias().alias_id(), alias_id_1);
                assert_eq!(output.as_alias().unlock_conditions().len(), 2);
                assert_eq!(output.as_alias().features().len(), 0);
                assert_eq!(output.as_alias().immutable_features().len(), 0);
                assert_eq!(
                    *output.as_alias().state_controller_address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
                assert_eq!(
                    *output.as_alias().governor_address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
            } else {
                panic!("unexpected output type")
            }
        }
    });
}

#[test]
fn burn_foundry_absent() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let foundry_id_1 = build_inputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )])[0]
        .output
        .as_foundry()
        .id();

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS_ED25519_0, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .burn(Burn::new().add_foundry(foundry_id_1))
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Foundry(foundry_id))) if foundry_id == foundry_id_1
    ));
}
