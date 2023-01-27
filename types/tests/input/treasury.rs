// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_types::block::{
    input::{Input, TreasuryInput},
    payload::milestone::MilestoneId,
};
use packable::PackableExt;

const MILESTONE_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";

#[test]
fn kind() {
    assert_eq!(TreasuryInput::KIND, 1);

    let input = Input::from(TreasuryInput::from_str(MILESTONE_ID).unwrap());

    assert_eq!(input.kind(), TreasuryInput::KIND);
}

#[test]
fn is_methods() {
    let input = Input::from(TreasuryInput::from_str(MILESTONE_ID).unwrap());

    assert!(!input.is_utxo());
    assert!(input.is_treasury());
}

#[test]
fn as_methods() {
    let treasury_input = TreasuryInput::from_str(MILESTONE_ID).unwrap();
    let input = Input::from(treasury_input);

    assert!(std::panic::catch_unwind(|| input.as_utxo()).is_err());
    assert_eq!(input.as_treasury(), &treasury_input);
}

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", TreasuryInput::from_str(MILESTONE_ID).unwrap()),
        "TreasuryInput(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649)"
    );
}

#[test]
fn new_valid() {
    let milestone_id = MilestoneId::from_str(MILESTONE_ID).unwrap();
    let input = TreasuryInput::new(milestone_id);

    assert_eq!(*input.milestone_id(), milestone_id);
    assert_eq!(*input, milestone_id);
}

#[test]
fn from_valid() {
    let milestone_id = MilestoneId::from_str(MILESTONE_ID).unwrap();
    let input: TreasuryInput = milestone_id.into();

    assert_eq!(*input.milestone_id(), milestone_id);
    assert_eq!(*input, milestone_id);
}

#[test]
fn from_str_valid() {
    let milestone_id = MilestoneId::from_str(MILESTONE_ID).unwrap();
    let input = TreasuryInput::from_str(MILESTONE_ID).unwrap();

    assert_eq!(*input.milestone_id(), milestone_id);
    assert_eq!(*input, milestone_id);
}

#[test]
fn from_str_to_str() {
    assert_eq!(TreasuryInput::from_str(MILESTONE_ID).unwrap().to_string(), MILESTONE_ID);
}

#[test]
fn packed_len() {
    let treasury_input = TreasuryInput::new(MilestoneId::from_str(MILESTONE_ID).unwrap());

    assert_eq!(treasury_input.packed_len(), 32);
    assert_eq!(treasury_input.pack_to_vec().len(), 32);
}

#[test]
fn pack_unpack_valid() {
    let input_1 = TreasuryInput::new(MilestoneId::from_str(MILESTONE_ID).unwrap());
    let input_2 = TreasuryInput::unpack_verified(input_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(input_1, input_2);
}
