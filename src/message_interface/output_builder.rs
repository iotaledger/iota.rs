// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    bee_block::{
        output::{
            dto::{AliasIdDto, NativeTokenDto, NftIdDto, OutputDto, TokenSchemeDto},
            feature::dto::FeatureDto,
            unlock_condition::dto::UnlockConditionDto,
            AliasId, AliasOutputBuilder, BasicOutputBuilder, Feature, FoundryOutputBuilder, NativeToken, NftId,
            NftOutputBuilder, TokenScheme, UnlockCondition,
        },
        DtoError,
    },
    Client, Result,
};

/// Alias output builder for the message interface
#[allow(clippy::too_many_arguments)]
pub async fn build_alias_output(
    client: &Client,
    // If not provided, minimum storage deposit will be used
    amount: Option<String>,
    native_tokens: Option<Vec<NativeTokenDto>>,
    alias_id: &AliasIdDto,
    state_index: Option<u32>,
    state_metadata: Option<Vec<u8>>,
    foundry_counter: Option<u32>,
    unlock_conditions: Vec<UnlockConditionDto>,
    features: Option<Vec<FeatureDto>>,
    immutable_features: Option<Vec<FeatureDto>>,
) -> Result<OutputDto> {
    let alias_id = AliasId::try_from(alias_id)?;

    let mut builder: AliasOutputBuilder;
    if let Some(amount) = amount {
        builder = AliasOutputBuilder::new_with_amount(
            amount.parse::<u64>().map_err(|_| DtoError::InvalidField("amount"))?,
            alias_id,
        )?;
    } else {
        // Config Builder
        let byte_cost_config = client.get_byte_cost_config().await?;
        builder = AliasOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config, alias_id)?;
    }

    if let Some(native_tokens) = native_tokens {
        let tokens = native_tokens
            .iter()
            .map(|native_token| Ok(NativeToken::try_from(native_token)?))
            .collect::<Result<Vec<NativeToken>>>()?;
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

    let conditions = unlock_conditions
        .iter()
        .map(|unlock_condition| Ok(UnlockCondition::try_from(unlock_condition)?))
        .collect::<Result<Vec<UnlockCondition>>>()?;
    builder = builder.with_unlock_conditions(conditions);

    if let Some(features) = features {
        let features = features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_features(features);
    }

    if let Some(immutable_features) = immutable_features {
        let immutable_features = immutable_features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_immutable_features(immutable_features);
    }

    let alias_output = builder.finish_output()?;

    // Convert to Dto
    Ok(OutputDto::from(&alias_output))
}

/// Basic output builder for the message interface
pub async fn build_basic_output(
    client: &Client,
    // If not provided, minimum storage deposit will be used
    amount: Option<String>,
    native_tokens: Option<Vec<NativeTokenDto>>,
    unlock_conditions: Vec<UnlockConditionDto>,
    features: Option<Vec<FeatureDto>>,
) -> Result<OutputDto> {
    let mut builder: BasicOutputBuilder;
    if let Some(amount) = amount {
        builder =
            BasicOutputBuilder::new_with_amount(amount.parse::<u64>().map_err(|_| DtoError::InvalidField("amount"))?)?;
    } else {
        // Config Builder
        let byte_cost_config = client.get_byte_cost_config().await?;
        builder = BasicOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config)?;
    }

    if let Some(native_tokens) = native_tokens {
        let tokens = native_tokens
            .iter()
            .map(|native_token| Ok(NativeToken::try_from(native_token)?))
            .collect::<Result<Vec<NativeToken>>>()?;
        builder = builder.with_native_tokens(tokens);
    }

    let conditions = unlock_conditions
        .iter()
        .map(|unlock_condition| Ok(UnlockCondition::try_from(unlock_condition)?))
        .collect::<Result<Vec<UnlockCondition>>>()?;
    builder = builder.with_unlock_conditions(conditions);

    if let Some(features) = features {
        let features = features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_features(features);
    }

    let basic_output = builder.finish_output()?;

    // Convert to Dto
    Ok(OutputDto::from(&basic_output))
}

/// Foundry output builder for the message interface
#[allow(clippy::too_many_arguments)]
pub async fn build_foundry_output(
    client: &Client,
    // If not provided, minimum storage deposit will be used
    amount: Option<String>,
    native_tokens: Option<Vec<NativeTokenDto>>,
    serial_number: u32,
    token_scheme: &TokenSchemeDto,
    unlock_conditions: Vec<UnlockConditionDto>,
    features: Option<Vec<FeatureDto>>,
    immutable_features: Option<Vec<FeatureDto>>,
) -> Result<OutputDto> {
    let token_scheme = TokenScheme::try_from(token_scheme)?;

    let mut builder: FoundryOutputBuilder;

    if let Some(amount) = amount {
        builder = FoundryOutputBuilder::new_with_amount(
            amount.parse::<u64>().map_err(|_| DtoError::InvalidField("amount"))?,
            serial_number,
            token_scheme,
        )?;
    } else {
        // Config Builder
        let byte_cost_config = client.get_byte_cost_config().await?;
        builder =
            FoundryOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config, serial_number, token_scheme)?;
    }

    if let Some(native_tokens) = native_tokens {
        let tokens = native_tokens
            .iter()
            .map(|native_token| Ok(NativeToken::try_from(native_token)?))
            .collect::<Result<Vec<NativeToken>>>()?;
        builder = builder.with_native_tokens(tokens);
    }

    let conditions = unlock_conditions
        .iter()
        .map(|unlock_condition| Ok(UnlockCondition::try_from(unlock_condition)?))
        .collect::<Result<Vec<UnlockCondition>>>()?;
    builder = builder.with_unlock_conditions(conditions);

    if let Some(features) = features {
        let features = features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_features(features);
    }

    if let Some(immutable_features) = immutable_features {
        let immutable_features = immutable_features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_immutable_features(immutable_features);
    }

    let foundry_output = builder.finish_output()?;

    // Convert to Dto
    Ok(OutputDto::from(&foundry_output))
}

/// Nft output builder for the message interface
pub async fn build_nft_output(
    client: &Client,
    // If not provided, minimum storage deposit will be used
    amount: Option<String>,
    native_tokens: Option<Vec<NativeTokenDto>>,
    nft_id: &NftIdDto,
    unlock_conditions: Vec<UnlockConditionDto>,
    features: Option<Vec<FeatureDto>>,
    immutable_features: Option<Vec<FeatureDto>>,
) -> Result<OutputDto> {
    let nft_id = NftId::try_from(nft_id)?;
    let mut builder: NftOutputBuilder;

    if let Some(amount) = amount {
        builder = NftOutputBuilder::new_with_amount(
            amount.parse::<u64>().map_err(|_| DtoError::InvalidField("amount"))?,
            nft_id,
        )?;
    } else {
        // Config Builder
        let byte_cost_config = client.get_byte_cost_config().await?;
        builder = NftOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config, nft_id)?;
    }

    if let Some(native_tokens) = native_tokens {
        let tokens = native_tokens
            .iter()
            .map(|native_token| Ok(NativeToken::try_from(native_token)?))
            .collect::<Result<Vec<NativeToken>>>()?;
        builder = builder.with_native_tokens(tokens);
    }

    let conditions = unlock_conditions
        .iter()
        .map(|unlock_condition| Ok(UnlockCondition::try_from(unlock_condition)?))
        .collect::<Result<Vec<UnlockCondition>>>()?;
    builder = builder.with_unlock_conditions(conditions);

    if let Some(features) = features {
        let features = features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_features(features);
    }

    if let Some(immutable_features) = immutable_features {
        let immutable_features = immutable_features
            .iter()
            .map(|feature| Ok(Feature::try_from(feature)?))
            .collect::<Result<Vec<Feature>>>()?;
        builder = builder.with_immutable_features(immutable_features);
    }

    let nft_output = builder.finish_output()?;

    // Convert to Dto
    Ok(OutputDto::from(&nft_output))
}
