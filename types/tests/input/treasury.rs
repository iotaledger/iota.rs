// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_types::block::{
    input::{
        dto::{InputDto, TreasuryInputDto},
        Input, TreasuryInput,
    },
    payload::milestone::MilestoneId,
    DtoError,
};
use packable::PackableExt;

const MILESTONE_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const MILESTONE_ID_INVALID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64";

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
fn new_milestone_id() {
    let milestone_id = MilestoneId::from_str(MILESTONE_ID).unwrap();
    let input = TreasuryInput::new(milestone_id);

    assert_eq!(*input.milestone_id(), milestone_id);
    assert_eq!(*input, milestone_id);
}

#[test]
fn from() {
    let milestone_id = MilestoneId::from_str(MILESTONE_ID).unwrap();
    let input = TreasuryInput::from(milestone_id);

    assert_eq!(*input.milestone_id(), milestone_id);
    assert_eq!(*input, milestone_id);
}

#[test]
fn from_str_to_str() {
    assert_eq!(TreasuryInput::from_str(MILESTONE_ID).unwrap().to_string(), MILESTONE_ID);
}

#[test]
fn debug() {
    assert_eq!(
        format!("{:?}", TreasuryInput::from_str(MILESTONE_ID).unwrap()),
        "TreasuryInput(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649)"
    );
}

#[test]
fn from_str() {
    let milestone_id = MilestoneId::from_str(MILESTONE_ID).unwrap();
    let input = TreasuryInput::from_str(MILESTONE_ID).unwrap();

    assert_eq!(*input.milestone_id(), milestone_id);
    assert_eq!(*input, milestone_id);
}

#[test]
fn dto_fields() {
    let treasury_input = TreasuryInput::from_str(MILESTONE_ID).unwrap();
    let treasury_dto = TreasuryInputDto::from(&treasury_input);

    assert_eq!(treasury_dto.kind, TreasuryInput::KIND);
    assert_eq!(treasury_dto.milestone_id, MILESTONE_ID.to_string());

    let input = Input::from(treasury_input);
    let dto = InputDto::from(&input);

    assert_eq!(dto, InputDto::Treasury(treasury_dto));
}

#[test]
fn dto_roundtrip() {
    let treasury_input = TreasuryInput::from_str(MILESTONE_ID).unwrap();
    let treasury_dto = TreasuryInputDto::from(&treasury_input);

    assert_eq!(TreasuryInput::try_from(&treasury_dto).unwrap(), treasury_input);

    let input = Input::from(treasury_input);
    let dto = InputDto::from(&input);

    assert_eq!(Input::try_from(&dto).unwrap(), input);
}

#[test]
fn dto_invalid() {
    let dto = TreasuryInputDto {
        kind: TreasuryInput::KIND,
        milestone_id: MILESTONE_ID_INVALID.to_string(),
    };

    assert!(matches!(
        TreasuryInput::try_from(&dto),
        Err(DtoError::InvalidField("milestoneId"))
    ));
}

#[test]
fn packed_len() {
    let treasury_input = TreasuryInput::from_str(MILESTONE_ID).unwrap();

    assert_eq!(treasury_input.packed_len(), MilestoneId::LENGTH);
    assert_eq!(treasury_input.pack_to_vec().len(), MilestoneId::LENGTH);

    let input = Input::from(treasury_input);

    assert_eq!(input.packed_len(), 1 + MilestoneId::LENGTH);
    assert_eq!(input.pack_to_vec().len(), 1 + MilestoneId::LENGTH);
}

#[test]
fn pack_unpack_valid() {
    let treasury_input = TreasuryInput::from_str(MILESTONE_ID).unwrap();
    let packed_input = treasury_input.pack_to_vec();

    assert_eq!(
        treasury_input,
        TreasuryInput::unpack_verified(packed_input.as_slice(), &()).unwrap(),
    );

    let input = Input::from(treasury_input);
    let packed_input = input.pack_to_vec();

    assert_eq!(input, Input::unpack_verified(packed_input.as_slice(), &()).unwrap(),);
}
