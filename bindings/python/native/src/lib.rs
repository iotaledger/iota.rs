// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! # Python binding implementation for Client library.

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

/// The client library of python binding.
pub mod types;
use std::sync::Mutex;

use iota_client::{
    bee_block::output::{
        dto::OutputDto, AliasId, AliasOutputBuilder, BasicOutputBuilder, ByteCostConfigBuilder, Feature,
        FoundryOutputBuilder, NativeToken, NftId, NftOutputBuilder, TokenScheme, UnlockCondition,
    },
    message_interface::MessageType,
};
use once_cell::sync::OnceCell;
use pyo3::{prelude::*, wrap_pyfunction};
use tokio::runtime::Runtime;
use types::*;

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

#[pyfunction]
/// Create message handler for python-side usage.
pub fn create_message_handler(options: Option<String>) -> Result<ClientMessageHandler> {
    let message_handler =
        crate::block_on(async { iota_client::message_interface::create_message_handler(options).await })?;

    Ok(ClientMessageHandler {
        client_message_handler: message_handler,
    })
}

#[pyfunction]
/// Send message through handler.
pub fn send_message(handle: &ClientMessageHandler, message_type: String) -> Result<String> {
    let message_type = match serde_json::from_str::<MessageType>(&message_type) {
        Ok(message_type) => message_type,
        Err(e) => {
            panic!("Wrong message type! {:?}", e);
        }
    };
    let response = crate::block_on(async {
        iota_client::message_interface::send_message(&handle.client_message_handler, message_type).await
    });
    Ok(serde_json::to_string(&response)?)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
/// Create OutputDto::Basic JSON string
pub fn create_basic_output(
    amount: Option<u64>,
    byte_cost: Option<u64>,
    key_factor: Option<u64>,
    data_factor: Option<u64>,
    native_tokens: Option<Vec<String>>,
    unlock_conditions: Option<Vec<String>>,
    features: Option<Vec<String>>,
) -> Result<String> {
    let mut builder: BasicOutputBuilder;
    if let Some(amount) = amount {
        builder = BasicOutputBuilder::new_with_amount(amount)?;
    } else {
        // Config Builder
        let mut config_builder = ByteCostConfigBuilder::new();
        if let Some(byte_cost) = byte_cost {
            config_builder = config_builder.byte_cost(byte_cost)
        }
        if let Some(key_factor) = key_factor {
            config_builder = config_builder.key_factor(key_factor)
        }
        if let Some(data_factor) = data_factor {
            config_builder = config_builder.data_factor(data_factor)
        }
        let config = config_builder.finish();
        builder = BasicOutputBuilder::new_with_minimum_storage_deposit(config)?;
    }
    if let Some(native_tokens) = native_tokens {
        let tokens: Vec<NativeToken> = native_tokens
            .iter()
            .map(|native_token| {
                serde_json::from_str::<NativeToken>(native_token)
                    .unwrap_or_else(|_| panic!("Invalid NativeToken: {:?}", native_token))
            })
            .collect();
        builder = builder.with_native_tokens(tokens);
    }
    if let Some(unlock_conditions) = unlock_conditions {
        let conditions: Vec<UnlockCondition> = unlock_conditions
            .iter()
            .map(|unlock_condition| {
                serde_json::from_str::<UnlockCondition>(unlock_condition)
                    .unwrap_or_else(|_| panic!("Invalid UnlockCondition: {:?}", unlock_condition))
            })
            .collect();
        builder = builder.with_unlock_conditions(conditions);
    }
    if let Some(features) = features {
        let blocks: Vec<Feature> = features
            .iter()
            .map(|feature| {
                serde_json::from_str::<Feature>(feature).unwrap_or_else(|_| panic!("Invalid Feature: {:?}", feature))
            })
            .collect();
        builder = builder.with_features(blocks);
    }
    let basic_output = &builder.finish_output()?;
    // Convert to Dto
    let basic_output_dto: OutputDto = basic_output.try_into()?;
    Ok(serde_json::ser::to_string(&basic_output_dto)?)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
/// Create OutputDto::Alias JSON string
pub fn create_alias_output(
    alias_id: String,
    amount: Option<u64>,
    byte_cost: Option<u64>,
    key_factor: Option<u64>,
    data_factor: Option<u64>,
    native_tokens: Option<Vec<String>>,
    state_index: Option<u32>,
    state_metadata: Option<Vec<u8>>,
    foundry_counter: Option<u32>,
    unlock_conditions: Option<Vec<String>>,
    features: Option<Vec<String>>,
    immutable_features: Option<Vec<String>>,
) -> Result<String> {
    let id = serde_json::from_str::<AliasId>(&alias_id).unwrap_or_else(|_| panic!("Invalid AliasId: {:?}", alias_id));
    let mut builder: AliasOutputBuilder;
    if let Some(amount) = amount {
        builder = AliasOutputBuilder::new_with_amount(amount, id)?;
    } else {
        // Config Builder
        let mut config_builder = ByteCostConfigBuilder::new();
        if let Some(byte_cost) = byte_cost {
            config_builder = config_builder.byte_cost(byte_cost)
        }
        if let Some(key_factor) = key_factor {
            config_builder = config_builder.key_factor(key_factor)
        }
        if let Some(data_factor) = data_factor {
            config_builder = config_builder.data_factor(data_factor)
        }
        let config = config_builder.finish();
        builder = AliasOutputBuilder::new_with_minimum_storage_deposit(config, id)?;
    }
    if let Some(native_tokens) = native_tokens {
        let tokens: Vec<NativeToken> = native_tokens
            .iter()
            .map(|native_token| {
                serde_json::from_str::<NativeToken>(native_token)
                    .unwrap_or_else(|_| panic!("Invalid NativeToken: {:?}", native_token))
            })
            .collect();
        builder = builder.with_native_tokens(tokens);
    }
    if let Some(state_index) = state_index {
        builder = builder.with_state_index(state_index);
    }
    if let Some(state_metadata) = state_metadata {
        builder = builder.with_state_metadata(state_metadata);
    }
    if let Some(foundry_counter) = foundry_counter {
        builder = builder.with_foundry_counter(foundry_counter);
    }
    if let Some(unlock_conditions) = unlock_conditions {
        let conditions: Vec<UnlockCondition> = unlock_conditions
            .iter()
            .map(|unlock_condition| {
                serde_json::from_str::<UnlockCondition>(unlock_condition)
                    .unwrap_or_else(|_| panic!("Invalid UnlockCondition: {:?}", unlock_condition))
            })
            .collect();
        builder = builder.with_unlock_conditions(conditions);
    }
    if let Some(features) = features {
        let blocks: Vec<Feature> = features
            .iter()
            .map(|feature| {
                serde_json::from_str::<Feature>(feature).unwrap_or_else(|_| panic!("Invalid Feature: {:?}", feature))
            })
            .collect();
        builder = builder.with_features(blocks);
    }
    if let Some(immutable_features) = immutable_features {
        let blocks: Vec<Feature> = immutable_features
            .iter()
            .map(|immutable_feature| {
                serde_json::from_str::<Feature>(immutable_feature)
                    .unwrap_or_else(|_| panic!("Invalid immutable Feature: {:?}", immutable_feature))
            })
            .collect();
        builder = builder.with_immutable_features(blocks);
    }
    let alias_output = &builder.finish_output()?;
    // Convert to Dto
    let alias_output_dto: OutputDto = alias_output.try_into()?;
    Ok(serde_json::ser::to_string(&alias_output_dto)?)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
/// Create OutputDto::Foundry JSON string
pub fn create_foundry_output(
    serial_number: u32,
    token_scheme: String,
    amount: Option<u64>,
    byte_cost: Option<u64>,
    key_factor: Option<u64>,
    data_factor: Option<u64>,
    native_tokens: Option<Vec<String>>,
    unlock_conditions: Option<Vec<String>>,
    features: Option<Vec<String>>,
    immutable_features: Option<Vec<String>>,
) -> Result<String> {
    let scheme = serde_json::from_str::<TokenScheme>(&token_scheme)
        .unwrap_or_else(|_| panic!("Invalid TokenScheme: {:?}", token_scheme));
    let mut builder: FoundryOutputBuilder;
    if let Some(amount) = amount {
        builder = FoundryOutputBuilder::new_with_amount(amount, serial_number, scheme)?;
    } else {
        // Config Builder
        let mut config_builder = ByteCostConfigBuilder::new();
        if let Some(byte_cost) = byte_cost {
            config_builder = config_builder.byte_cost(byte_cost)
        }
        if let Some(key_factor) = key_factor {
            config_builder = config_builder.key_factor(key_factor)
        }
        if let Some(data_factor) = data_factor {
            config_builder = config_builder.data_factor(data_factor)
        }
        let config = config_builder.finish();
        builder = FoundryOutputBuilder::new_with_minimum_storage_deposit(config, serial_number, scheme)?;
    }
    if let Some(native_tokens) = native_tokens {
        let tokens: Vec<NativeToken> = native_tokens
            .iter()
            .map(|native_token| {
                serde_json::from_str::<NativeToken>(native_token)
                    .unwrap_or_else(|_| panic!("Invalid NativeToken: {:?}", native_token))
            })
            .collect();
        builder = builder.with_native_tokens(tokens);
    }
    if let Some(unlock_conditions) = unlock_conditions {
        let conditions: Vec<UnlockCondition> = unlock_conditions
            .iter()
            .map(|unlock_condition| {
                serde_json::from_str::<UnlockCondition>(unlock_condition)
                    .unwrap_or_else(|_| panic!("Invalid UnlockCondition: {:?}", unlock_condition))
            })
            .collect();
        builder = builder.with_unlock_conditions(conditions);
    }
    if let Some(features) = features {
        let blocks: Vec<Feature> = features
            .iter()
            .map(|feature| {
                serde_json::from_str::<Feature>(feature).unwrap_or_else(|_| panic!("Invalid Feature: {:?}", feature))
            })
            .collect();
        builder = builder.with_features(blocks);
    }
    if let Some(immutable_features) = immutable_features {
        let blocks: Vec<Feature> = immutable_features
            .iter()
            .map(|immutable_feature| {
                serde_json::from_str::<Feature>(immutable_feature)
                    .unwrap_or_else(|_| panic!("Invalid immutable Feature: {:?}", immutable_feature))
            })
            .collect();
        builder = builder.with_immutable_features(blocks);
    }
    let foundry_output = &builder.finish_output()?;
    // Convert to Dto
    let foundry_output_dto: OutputDto = foundry_output.try_into()?;
    Ok(serde_json::ser::to_string(&foundry_output_dto)?)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
/// Create OutputDto::Nft JSON string
pub fn create_nft_output(
    nft_id: String,
    amount: Option<u64>,
    byte_cost: Option<u64>,
    key_factor: Option<u64>,
    data_factor: Option<u64>,
    native_tokens: Option<Vec<String>>,
    unlock_conditions: Option<Vec<String>>,
    features: Option<Vec<String>>,
    immutable_features: Option<Vec<String>>,
) -> Result<String> {
    let id = serde_json::from_str::<NftId>(&nft_id).unwrap_or_else(|_| panic!("Invalid NftId: {:?}", nft_id));
    let mut builder: NftOutputBuilder;
    if let Some(amount) = amount {
        builder = NftOutputBuilder::new_with_amount(amount, id)?;
    } else {
        // Config Builder
        let mut config_builder = ByteCostConfigBuilder::new();
        if let Some(byte_cost) = byte_cost {
            config_builder = config_builder.byte_cost(byte_cost)
        }
        if let Some(key_factor) = key_factor {
            config_builder = config_builder.key_factor(key_factor)
        }
        if let Some(data_factor) = data_factor {
            config_builder = config_builder.data_factor(data_factor)
        }
        let config = config_builder.finish();
        builder = NftOutputBuilder::new_with_minimum_storage_deposit(config, id)?;
    }
    if let Some(native_tokens) = native_tokens {
        let tokens: Vec<NativeToken> = native_tokens
            .iter()
            .map(|native_token| {
                serde_json::from_str::<NativeToken>(native_token)
                    .unwrap_or_else(|_| panic!("Invalid NativeToken: {:?}", native_token))
            })
            .collect();
        builder = builder.with_native_tokens(tokens);
    }
    if let Some(unlock_conditions) = unlock_conditions {
        let conditions: Vec<UnlockCondition> = unlock_conditions
            .iter()
            .map(|unlock_condition| {
                serde_json::from_str::<UnlockCondition>(unlock_condition)
                    .unwrap_or_else(|_| panic!("Invalid UnlockCondition: {:?}", unlock_condition))
            })
            .collect();
        builder = builder.with_unlock_conditions(conditions);
    }
    if let Some(features) = features {
        let blocks: Vec<Feature> = features
            .iter()
            .map(|feature| {
                serde_json::from_str::<Feature>(feature).unwrap_or_else(|_| panic!("Invalid Feature: {:?}", feature))
            })
            .collect();
        builder = builder.with_features(blocks);
    }
    if let Some(immutable_features) = immutable_features {
        let blocks: Vec<Feature> = immutable_features
            .iter()
            .map(|immutable_feature| {
                serde_json::from_str::<Feature>(immutable_feature)
                    .unwrap_or_else(|_| panic!("Invalid immutable Feature: {:?}", immutable_feature))
            })
            .collect();
        builder = builder.with_immutable_features(blocks);
    }
    let nft_output = &builder.finish_output()?;
    // Convert to Dto
    let nft_output_dto: OutputDto = nft_output.try_into()?;
    Ok(serde_json::ser::to_string(&nft_output_dto)?)
}

/// IOTA Client implemented in Rust for Python binding.
#[pymodule]
fn iota_client(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_message_handler, m)?).unwrap();
    m.add_function(wrap_pyfunction!(send_message, m)?).unwrap();
    m.add_function(wrap_pyfunction!(create_basic_output, m)?).unwrap();
    m.add_function(wrap_pyfunction!(create_alias_output, m)?).unwrap();
    m.add_function(wrap_pyfunction!(create_foundry_output, m)?).unwrap();
    m.add_function(wrap_pyfunction!(create_nft_output, m)?).unwrap();
    Ok(())
}
