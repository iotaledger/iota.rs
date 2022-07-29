// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_block::output::unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition};
use iota_client::{
    block::{
        address::Address,
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            AliasId, AliasOutputBuilder, BasicOutputBuilder, NftId, NftOutputBuilder, Output,
        },
        rand::{block::rand_block_id, transaction::rand_transaction_id},
    },
    secret::types::{InputSigningData, OutputMetadata},
};

mod alias_foundry_outputs;
mod basic_outputs;
mod nft_outputs;

fn build_most_basic_output(bech32_address: &str, amount: u64) -> Output {
    BasicOutputBuilder::new_with_amount(amount)
        .unwrap()
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
            Address::try_from_bech32(bech32_address).unwrap().1,
        )))
        .finish_output()
        .unwrap()
}

fn build_nft_output(nft_id: NftId, bech32_address: &str, amount: u64) -> Output {
    NftOutputBuilder::new_with_amount(amount, nft_id)
        .unwrap()
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
            Address::try_from_bech32(bech32_address).unwrap().1,
        )))
        .finish_output()
        .unwrap()
}

fn build_alias_output(alias_id: AliasId, bech32_address: &str, amount: u64) -> Output {
    let address = Address::try_from_bech32(bech32_address).unwrap().1;
    AliasOutputBuilder::new_with_amount(amount, alias_id)
        .unwrap()
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
            StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
            address,
        )))
        .finish_output()
        .unwrap()
}

fn build_input_signing_data_most_basic_outputs(outputs: Vec<(&str, u64)>) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(bech32_address, amount)| InputSigningData {
            output: build_most_basic_output(bech32_address, amount),
            output_metadata: OutputMetadata {
                block_id: rand_block_id(),
                transaction_id: rand_transaction_id(),
                output_index: 0,
                is_spent: false,
                milestone_index_spent: None,
                milestone_timestamp_spent: None,
                transaction_id_spent: None,
                milestone_index_booked: 0,
                milestone_timestamp_booked: 0,
                ledger_index: 0,
            },
            chain: None,
            bech32_address: String::new(),
        })
        .collect()
}

fn build_input_signing_data_nft_outputs(outputs: Vec<(NftId, &str, u64)>) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(nft_id, bech32_address, amount)| InputSigningData {
            output: build_nft_output(nft_id, bech32_address, amount),
            output_metadata: OutputMetadata {
                block_id: rand_block_id(),
                transaction_id: rand_transaction_id(),
                output_index: 0,
                is_spent: false,
                milestone_index_spent: None,
                milestone_timestamp_spent: None,
                transaction_id_spent: None,
                milestone_index_booked: 0,
                milestone_timestamp_booked: 0,
                ledger_index: 0,
            },
            chain: None,
            bech32_address: String::new(),
        })
        .collect()
}

fn build_input_signing_data_alias_outputs(outputs: Vec<(AliasId, &str, u64)>) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(alias_id, bech32_address, amount)| InputSigningData {
            output: build_alias_output(alias_id, bech32_address, amount),
            output_metadata: OutputMetadata {
                block_id: rand_block_id(),
                transaction_id: rand_transaction_id(),
                output_index: 0,
                is_spent: false,
                milestone_index_spent: None,
                milestone_timestamp_spent: None,
                transaction_id_spent: None,
                milestone_index_booked: 0,
                milestone_timestamp_booked: 0,
                ledger_index: 0,
            },
            chain: None,
            bech32_address: String::new(),
        })
        .collect()
}
