// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::type_complexity)]

mod alias_outputs;
mod basic_outputs;
mod burn;
mod foundry_outputs;
mod native_tokens;
mod nft_outputs;
mod outputs;

use std::{collections::HashMap, hash::Hash, str::FromStr};

use iota_client::{
    block::{
        address::{Address, AliasAddress},
        output::{
            feature::{Feature, IssuerFeature, SenderFeature},
            unlock_condition::{
                AddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAliasAddressUnlockCondition,
                StateControllerAddressUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, NativeToken, NftId,
            NftOutputBuilder, Output, OutputId, SimpleTokenScheme, TokenId, TokenScheme,
        },
        rand::{block::rand_block_id, transaction::rand_transaction_id},
    },
    constants::SHIMMER_TESTNET_BECH32_HRP,
    secret::types::{InputSigningData, OutputMetadata},
};
use primitive_types::U256;

const TOKEN_SUPPLY: u64 = 1_813_620_509_061_365;
const ALIAS_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const ALIAS_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";
const ALIAS_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222";
const NFT_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const NFT_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";
const NFT_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222";
const TOKEN_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111111111111111";
const TOKEN_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222222222222222";
const BECH32_ADDRESS: &str = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";
const BECH32_ADDRESS_REMAINDER: &str = "rms1qrut5ajyfrtgjs325kd9chwfwyyy2z3fewy4vgy0vvdtf2pr8prg5u3zwjn";
const BECH32_ADDRESS_ED25519_SENDER: &str = "rms1qqhvvur9xfj6yhgsxfa4f8xst7vz9zxeu3vcxds8mh4a6jlpteq9xrajhtf";
const BECH32_ADDRESS_ALIAS_SENDER: &str = "rms1pqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zws5524"; // Corresponds to ALIAS_ID_1
const BECH32_ADDRESS_NFT_SENDER: &str = "rms1zqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zxddmy7"; // Corresponds to NFT_ID_1

fn build_basic_output(
    amount: u64,
    bech32_address: &str,
    native_tokens: Option<Vec<(&str, u64)>>,
    bech32_sender: Option<&str>,
) -> Output {
    let mut builder = BasicOutputBuilder::new_with_amount(amount)
        .unwrap()
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
            Address::try_from_bech32(bech32_address).unwrap().1,
        )));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), U256::from(amount)).unwrap()),
        );
    }

    if let Some(bech32_sender) = bech32_sender {
        builder = builder.add_feature(Feature::Sender(SenderFeature::new(
            Address::try_from_bech32(bech32_sender).unwrap().1,
        )));
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

fn build_nft_output(
    amount: u64,
    nft_id: NftId,
    bech32_address: &str,
    native_tokens: Option<Vec<(&str, u64)>>,
    bech32_sender: Option<&str>,
    bech32_issuer: Option<&str>,
) -> Output {
    let mut builder = NftOutputBuilder::new_with_amount(amount, nft_id)
        .unwrap()
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
            Address::try_from_bech32(bech32_address).unwrap().1,
        )));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), U256::from(amount)).unwrap()),
        );
    }

    if let Some(bech32_sender) = bech32_sender {
        builder = builder.add_feature(Feature::Sender(SenderFeature::new(
            Address::try_from_bech32(bech32_sender).unwrap().1,
        )));
    }

    if let Some(bech32_issuer) = bech32_issuer {
        builder = builder.add_immutable_feature(Feature::Issuer(IssuerFeature::new(
            Address::try_from_bech32(bech32_issuer).unwrap().1,
        )));
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

fn build_alias_output(
    amount: u64,
    alias_id: AliasId,
    bech32_address: &str,
    native_tokens: Option<Vec<(&str, u64)>>,
    bech32_sender: Option<&str>,
    bech32_issuer: Option<&str>,
) -> Output {
    let address = Address::try_from_bech32(bech32_address).unwrap().1;

    let mut builder = AliasOutputBuilder::new_with_amount(amount, alias_id)
        .unwrap()
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
            StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
            address,
        )));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), U256::from(amount)).unwrap()),
        );
    }

    if let Some(bech32_sender) = bech32_sender {
        builder = builder.add_feature(Feature::Sender(SenderFeature::new(
            Address::try_from_bech32(bech32_sender).unwrap().1,
        )));
    }

    if let Some(bech32_issuer) = bech32_issuer {
        builder = builder.add_immutable_feature(Feature::Issuer(IssuerFeature::new(
            Address::try_from_bech32(bech32_issuer).unwrap().1,
        )));
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

fn build_foundry_output(
    amount: u64,
    alias_id: AliasId,
    token_scheme: SimpleTokenScheme,
    native_tokens: Option<Vec<(&str, u64)>>,
) -> Output {
    let mut builder = FoundryOutputBuilder::new_with_amount(amount, 0, TokenScheme::Simple(token_scheme))
        .unwrap()
        .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
            ImmutableAliasAddressUnlockCondition::new(AliasAddress::new(alias_id)),
        ));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), U256::from(amount)).unwrap()),
        );
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

fn build_input_signing_data_most_basic_outputs(
    outputs: Vec<(u64, &str, Option<Vec<(&str, u64)>>)>,
) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(amount, bech32_address, native_tokens)| InputSigningData {
            output: build_basic_output(amount, bech32_address, native_tokens, None),
            output_metadata: OutputMetadata::new(
                rand_block_id(),
                OutputId::new(rand_transaction_id(), 0).unwrap(),
                false,
                None,
                None,
                None,
                0,
                0,
                0,
            ),
            chain: None,
            bech32_address: bech32_address.to_string(),
        })
        .collect()
}

fn build_input_signing_data_nft_outputs(
    outputs: Vec<(u64, NftId, &str, Option<Vec<(&str, u64)>>)>,
) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(amount, nft_id, bech32_address, native_tokens)| InputSigningData {
            output: build_nft_output(amount, nft_id, bech32_address, native_tokens, None, None),
            output_metadata: OutputMetadata::new(
                rand_block_id(),
                OutputId::new(rand_transaction_id(), 0).unwrap(),
                false,
                None,
                None,
                None,
                0,
                0,
                0,
            ),
            chain: None,
            bech32_address: bech32_address.to_string(),
        })
        .collect()
}

fn build_input_signing_data_alias_outputs(
    outputs: Vec<(u64, AliasId, &str, Option<Vec<(&str, u64)>>)>,
) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(amount, alias_id, bech32_address, native_tokens)| InputSigningData {
            output: build_alias_output(amount, alias_id, bech32_address, native_tokens, None, None),
            output_metadata: OutputMetadata::new(
                rand_block_id(),
                OutputId::new(rand_transaction_id(), 0).unwrap(),
                false,
                None,
                None,
                None,
                0,
                0,
                0,
            ),
            chain: None,
            bech32_address: bech32_address.to_string(),
        })
        .collect()
}

fn build_input_signing_data_foundry_outputs(
    outputs: Vec<(u64, AliasId, SimpleTokenScheme, Option<Vec<(&str, u64)>>)>,
) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(
            |(amount, alias_id, simple_token_scheme, native_tokens)| InputSigningData {
                output: build_foundry_output(amount, alias_id, simple_token_scheme, native_tokens),
                output_metadata: OutputMetadata::new(
                    rand_block_id(),
                    OutputId::new(rand_transaction_id(), 0).unwrap(),
                    false,
                    None,
                    None,
                    None,
                    0,
                    0,
                    0,
                ),
                chain: None,
                bech32_address: Address::Alias(AliasAddress::new(alias_id)).to_bech32(SHIMMER_TESTNET_BECH32_HRP),
            },
        )
        .collect()
}

fn unsorted_eq<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    fn count<T>(items: &[T]) -> HashMap<&T, usize>
    where
        T: Eq + Hash,
    {
        let mut cnt = HashMap::new();
        for i in items {
            *cnt.entry(i).or_insert(0) += 1
        }
        cnt
    }

    count(a) == count(b)
}
