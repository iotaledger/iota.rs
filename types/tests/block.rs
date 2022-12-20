// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_pow::{miner::get_miner, score::PowScorer};
use iota_types::block::{
    parent::Parents,
    payload::{Payload, TaggedDataPayload},
    protocol::protocol_parameters,
    rand::{
        block::rand_block_ids,
        number::rand_number,
        parents::rand_parents,
        payload::{rand_tagged_data_payload, rand_treasury_transaction_payload},
    },
    Block, BlockBuilder, Error,
};
use packable::{error::UnpackError, PackableExt};

#[test]
fn default_finish_zero_nonce() {
    let block = BlockBuilder::new(rand_parents()).finish().unwrap();

    assert!(block.nonce() == 0);
}

#[test]
fn pow_provider() {
    let min_pow_score = protocol_parameters().min_pow_score();
    let block = BlockBuilder::new(rand_parents())
        .finish_nonce(get_miner(min_pow_score))
        .unwrap();

    let block_bytes = block.pack_to_vec();
    let score = PowScorer::new().score(&block_bytes);

    assert!(score >= min_pow_score as f64);
}

#[test]
fn invalid_length() {
    let res = BlockBuilder::new(Parents::new(rand_block_ids(2)).unwrap())
        .with_nonce(42)
        .with_payload(
            TaggedDataPayload::new(vec![42], vec![0u8; Block::LENGTH_MAX - Block::LENGTH_MIN - 9])
                .unwrap()
                .into(),
        )
        .finish();

    assert!(matches!(res, Err(Error::InvalidBlockLength(len)) if len == Block::LENGTH_MAX + 33));
}

#[test]
fn invalid_payload_kind() {
    let protocol_parameters = protocol_parameters();
    let res = BlockBuilder::new(rand_parents())
        .with_payload(rand_treasury_transaction_payload(protocol_parameters.token_supply()).into())
        .finish();

    assert!(matches!(res, Err(Error::InvalidPayloadKind(4))))
}

#[test]
fn unpack_valid_no_remaining_bytes() {
    assert!(
        Block::unpack_strict(
            vec![
                2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168, 202,
                82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62, 38, 20,
                121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0, 0, 0, 21,
                205, 91, 7, 0, 0, 0, 0,
            ]
            .as_slice(),
            &protocol_parameters()
        )
        .is_ok()
    )
}

#[test]
fn unpack_invalid_remaining_bytes() {
    assert!(matches!(
        Block::unpack_strict(
            vec![
                2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168, 202,
                82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62, 38, 20,
                121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0, 0, 0, 21,
                205, 91, 7, 0, 0, 0, 0, 42
            ]
            .as_slice(),
            &protocol_parameters()
        ),
        Err(UnpackError::Packable(Error::RemainingBytesAfterBlock))
    ))
}

// Validate that a `unpack` ∘ `pack` round-trip results in the original block.
#[test]
fn pack_unpack_valid() {
    let protocol_parameters = protocol_parameters();
    let block = BlockBuilder::new(rand_parents()).finish().unwrap();
    let packed_block = block.pack_to_vec();

    assert_eq!(packed_block.len(), block.packed_len());
    assert_eq!(
        block,
        PackableExt::unpack_verified(packed_block.as_slice(), &protocol_parameters).unwrap()
    );
}

#[test]
fn getters() {
    let protocol_parameters = protocol_parameters();
    let parents = rand_parents();
    let payload: Payload = rand_tagged_data_payload().into();
    let nonce: u64 = rand_number();

    let block = BlockBuilder::new(parents.clone())
        .with_payload(payload.clone())
        .with_nonce(nonce)
        .finish()
        .unwrap();

    assert_eq!(block.protocol_version(), protocol_parameters.protocol_version());
    assert_eq!(*block.parents(), parents);
    assert_eq!(*block.payload().as_ref().unwrap(), &payload);
    assert_eq!(block.nonce(), nonce);
}
