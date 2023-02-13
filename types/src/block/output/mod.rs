// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias_id;
mod chain_id;
mod foundry_id;
mod inputs_commitment;
mod metadata;
mod native_token;
mod nft_id;
mod output_id;
mod rent;
mod state_transition;
mod token_id;
mod token_scheme;
mod treasury;

///
pub mod alias;
///
pub mod basic;
///
pub mod feature;
///
pub mod foundry;
///
pub mod nft;
///
pub mod unlock_condition;

use core::ops::RangeInclusive;

use derive_more::From;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

pub(crate) use self::{
    alias::StateMetadataLength,
    feature::{MetadataFeatureLength, TagFeatureLength},
    native_token::NativeTokenCount,
    output_id::OutputIndex,
    unlock_condition::AddressUnlockCondition,
};
pub use self::{
    alias::{AliasOutput, AliasOutputBuilder, AliasTransition},
    alias_id::AliasId,
    basic::{BasicOutput, BasicOutputBuilder},
    chain_id::ChainId,
    feature::{Feature, Features},
    foundry::{FoundryOutput, FoundryOutputBuilder},
    foundry_id::FoundryId,
    inputs_commitment::InputsCommitment,
    metadata::OutputMetadata,
    native_token::{NativeToken, NativeTokens, NativeTokensBuilder},
    nft::{NftOutput, NftOutputBuilder},
    nft_id::NftId,
    output_id::OutputId,
    rent::{Rent, RentStructure, RentStructureBuilder},
    state_transition::{StateTransitionError, StateTransitionVerifier},
    token_id::TokenId,
    token_scheme::{SimpleTokenScheme, TokenScheme},
    treasury::TreasuryOutput,
    unlock_condition::{UnlockCondition, UnlockConditions},
};
use crate::block::{address::Address, protocol::ProtocolParameters, semantic::ValidationContext, Error};

/// The maximum number of outputs of a transaction.
pub const OUTPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of outputs of a transaction .
pub const OUTPUT_COUNT_RANGE: RangeInclusive<u16> = 1..=OUTPUT_COUNT_MAX; // [1..128]
/// The maximum index of outputs of a transaction.
pub const OUTPUT_INDEX_MAX: u16 = OUTPUT_COUNT_MAX - 1; // 127
/// The range of valid indices of outputs of a transaction .
pub const OUTPUT_INDEX_RANGE: RangeInclusive<u16> = 0..=OUTPUT_INDEX_MAX; // [0..127]

#[derive(Clone)]
pub(crate) enum OutputBuilderAmount {
    Amount(u64),
    MinimumStorageDeposit(RentStructure),
}

/// A generic output that can represent different types defining the deposit of funds.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
pub enum Output {
    /// A treasury output.
    Treasury(TreasuryOutput),
    /// A basic output.
    Basic(BasicOutput),
    /// An alias output.
    Alias(AliasOutput),
    /// A foundry output.
    Foundry(FoundryOutput),
    /// An NFT output.
    Nft(NftOutput),
}

impl Output {
    /// Minimum amount for an output.
    pub const AMOUNT_MIN: u64 = 1;

    /// Return the output kind of an [`Output`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Treasury(_) => TreasuryOutput::KIND,
            Self::Basic(_) => BasicOutput::KIND,
            Self::Alias(_) => AliasOutput::KIND,
            Self::Foundry(_) => FoundryOutput::KIND,
            Self::Nft(_) => NftOutput::KIND,
        }
    }

    /// Returns the amount of an [`Output`].
    pub fn amount(&self) -> u64 {
        match self {
            Self::Treasury(output) => output.amount(),
            Self::Basic(output) => output.amount(),
            Self::Alias(output) => output.amount(),
            Self::Foundry(output) => output.amount(),
            Self::Nft(output) => output.amount(),
        }
    }

    /// Returns the native tokens of an [`Output`], if any.
    pub fn native_tokens(&self) -> Option<&NativeTokens> {
        match self {
            Self::Treasury(_) => None,
            Self::Basic(output) => Some(output.native_tokens()),
            Self::Alias(output) => Some(output.native_tokens()),
            Self::Foundry(output) => Some(output.native_tokens()),
            Self::Nft(output) => Some(output.native_tokens()),
        }
    }

    /// Returns the unlock conditions of an [`Output`], if any.
    pub fn unlock_conditions(&self) -> Option<&UnlockConditions> {
        match self {
            Self::Treasury(_) => None,
            Self::Basic(output) => Some(output.unlock_conditions()),
            Self::Alias(output) => Some(output.unlock_conditions()),
            Self::Foundry(output) => Some(output.unlock_conditions()),
            Self::Nft(output) => Some(output.unlock_conditions()),
        }
    }

    /// Returns the features of an [`Output`], if any.
    pub fn features(&self) -> Option<&Features> {
        match self {
            Self::Treasury(_) => None,
            Self::Basic(output) => Some(output.features()),
            Self::Alias(output) => Some(output.features()),
            Self::Foundry(output) => Some(output.features()),
            Self::Nft(output) => Some(output.features()),
        }
    }

    /// Returns the immutable features of an [`Output`], if any.
    pub fn immutable_features(&self) -> Option<&Features> {
        match self {
            Self::Treasury(_) => None,
            Self::Basic(_) => None,
            Self::Alias(output) => Some(output.immutable_features()),
            Self::Foundry(output) => Some(output.immutable_features()),
            Self::Nft(output) => Some(output.immutable_features()),
        }
    }

    /// Returns the chain identifier of an [`Output`], if any.
    pub fn chain_id(&self) -> Option<ChainId> {
        match self {
            Self::Treasury(_) => None,
            Self::Basic(_) => None,
            Self::Alias(output) => Some(output.chain_id()),
            Self::Foundry(output) => Some(output.chain_id()),
            Self::Nft(output) => Some(output.chain_id()),
        }
    }

    /// Checks whether the output is a [`TreasuryOutput`].
    pub fn is_treasury(&self) -> bool {
        matches!(self, Self::Treasury(_))
    }

    /// Gets the output as an actual [`TreasuryOutput`].
    /// PANIC: do not call on a non-treasury output.
    pub fn as_treasury(&self) -> &TreasuryOutput {
        if let Self::Treasury(output) = self {
            output
        } else {
            panic!("as_treasury called on a non-treasury output");
        }
    }

    /// Checks whether the output is a [`BasicOutput`].
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic(_))
    }

    /// Gets the output as an actual [`BasicOutput`].
    /// PANIC: do not call on a non-basic output.
    pub fn as_basic(&self) -> &BasicOutput {
        if let Self::Basic(output) = self {
            output
        } else {
            panic!("as_basic called on a non-basic output");
        }
    }

    /// Checks whether the output is an [`AliasOutput`].
    pub fn is_alias(&self) -> bool {
        matches!(self, Self::Alias(_))
    }

    /// Gets the output as an actual [`AliasOutput`].
    /// PANIC: do not call on a non-alias output.
    pub fn as_alias(&self) -> &AliasOutput {
        if let Self::Alias(output) = self {
            output
        } else {
            panic!("as_alias called on a non-alias output");
        }
    }

    /// Checks whether the output is a [`FoundryOutput`].
    pub fn is_foundry(&self) -> bool {
        matches!(self, Self::Foundry(_))
    }

    /// Gets the output as an actual [`FoundryOutput`].
    /// PANIC: do not call on a non-foundry output.
    pub fn as_foundry(&self) -> &FoundryOutput {
        if let Self::Foundry(output) = self {
            output
        } else {
            panic!("as_foundry called on a non-foundry output");
        }
    }

    /// Checks whether the output is an [`NftOutput`].
    pub fn is_nft(&self) -> bool {
        matches!(self, Self::Nft(_))
    }

    /// Gets the output as an actual [`NftOutput`].
    /// PANIC: do not call on a non-nft output.
    pub fn as_nft(&self) -> &NftOutput {
        if let Self::Nft(output) = self {
            output
        } else {
            panic!("as_nft called on a non-nft output");
        }
    }

    /// Returns the address that is required to unlock this [`Output`] and the alias or nft address that gets
    /// unlocked by it, if it's an alias or nft.
    /// If no `alias_transition` has been provided, assumes a state transition.
    pub fn required_and_unlocked_address(
        &self,
        current_time: u32,
        output_id: &OutputId,
        alias_transition: Option<AliasTransition>,
    ) -> Result<(Address, Option<Address>), Error> {
        match self {
            Self::Alias(output) => {
                if alias_transition.unwrap_or(AliasTransition::State) == AliasTransition::State {
                    // Alias address is only unlocked if it's a state transition
                    Ok((
                        *output.state_controller_address(),
                        Some(Address::Alias(output.alias_address(output_id))),
                    ))
                } else {
                    Ok((*output.governor_address(), None))
                }
            }
            Self::Basic(output) => Ok((
                *output
                    .unlock_conditions()
                    .locked_address(output.address(), current_time),
                None,
            )),
            Self::Nft(output) => Ok((
                *output
                    .unlock_conditions()
                    .locked_address(output.address(), current_time),
                Some(Address::Nft(output.nft_address(output_id))),
            )),
            Self::Foundry(output) => Ok((Address::Alias(*output.alias_address()), None)),
            Self::Treasury(_) => Err(Error::UnsupportedOutputKind(TreasuryOutput::KIND)),
        }
    }

    ///
    pub fn verify_state_transition(
        current_state: Option<&Self>,
        next_state: Option<&Self>,
        context: &ValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        match (current_state, next_state) {
            // Creations.
            (None, Some(Self::Alias(next_state))) => AliasOutput::creation(next_state, context),
            (None, Some(Self::Foundry(next_state))) => FoundryOutput::creation(next_state, context),
            (None, Some(Self::Nft(next_state))) => NftOutput::creation(next_state, context),

            // Transitions.
            (Some(Self::Alias(current_state)), Some(Self::Alias(next_state))) => {
                AliasOutput::transition(current_state, next_state, context)
            }
            (Some(Self::Foundry(current_state)), Some(Self::Foundry(next_state))) => {
                FoundryOutput::transition(current_state, next_state, context)
            }
            (Some(Self::Nft(current_state)), Some(Self::Nft(next_state))) => {
                NftOutput::transition(current_state, next_state, context)
            }

            // Destructions.
            (Some(Self::Alias(current_state)), None) => AliasOutput::destruction(current_state, context),
            (Some(Self::Foundry(current_state)), None) => FoundryOutput::destruction(current_state, context),
            (Some(Self::Nft(current_state)), None) => NftOutput::destruction(current_state, context),

            // Unsupported.
            _ => Err(StateTransitionError::UnsupportedStateTransition),
        }
    }

    /// Verifies if a valid storage deposit was made. Each [`Output`] has to have an amount that covers its associated
    /// byte cost, given by [`RentStructure`].
    /// If there is a [`StorageDepositReturnUnlockCondition`](unlock_condition::StorageDepositReturnUnlockCondition),
    /// its amount is also checked.
    pub fn verify_storage_deposit(&self, rent_structure: RentStructure, token_supply: u64) -> Result<(), Error> {
        let required_output_amount = self.rent_cost(&rent_structure);

        if self.amount() < required_output_amount {
            return Err(Error::InsufficientStorageDepositAmount {
                amount: self.amount(),
                required: required_output_amount,
            });
        }

        if let Some(return_condition) = self
            .unlock_conditions()
            .and_then(UnlockConditions::storage_deposit_return)
        {
            // We can't return more tokens than were originally contained in the output.
            // `Return Amount` ≤ `Amount`.
            if return_condition.amount() > self.amount() {
                return Err(Error::StorageDepositReturnExceedsOutputAmount {
                    deposit: return_condition.amount(),
                    amount: self.amount(),
                });
            }

            let minimum_deposit =
                minimum_storage_deposit(return_condition.return_address(), rent_structure, token_supply);

            // `Minimum Storage Deposit` ≤ `Return Amount`
            if return_condition.amount() < minimum_deposit {
                return Err(Error::InsufficientStorageDepositReturnAmount {
                    deposit: return_condition.amount(),
                    required: minimum_deposit,
                });
            }
        }

        Ok(())
    }
}

impl Packable for Output {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Treasury(output) => {
                TreasuryOutput::KIND.pack(packer)?;
                output.pack(packer)
            }
            Self::Basic(output) => {
                BasicOutput::KIND.pack(packer)?;
                output.pack(packer)
            }
            Self::Alias(output) => {
                AliasOutput::KIND.pack(packer)?;
                output.pack(packer)
            }
            Self::Foundry(output) => {
                FoundryOutput::KIND.pack(packer)?;
                output.pack(packer)
            }
            Self::Nft(output) => {
                NftOutput::KIND.pack(packer)?;
                output.pack(packer)
            }
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            TreasuryOutput::KIND => Self::from(TreasuryOutput::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            BasicOutput::KIND => Self::from(BasicOutput::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            AliasOutput::KIND => Self::from(AliasOutput::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            FoundryOutput::KIND => Self::from(FoundryOutput::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            NftOutput::KIND => Self::from(NftOutput::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            k => return Err(Error::InvalidOutputKind(k)).map_err(UnpackError::Packable),
        })
    }
}

impl Rent for Output {
    fn weighted_bytes(&self, rent_structure: &RentStructure) -> u64 {
        self.packed_len() as u64 * rent_structure.byte_factor_data() as u64
    }
}

pub(crate) fn verify_output_amount<const VERIFY: bool>(amount: &u64, token_supply: &u64) -> Result<(), Error> {
    if VERIFY && (*amount < Output::AMOUNT_MIN || amount > token_supply) {
        Err(Error::InvalidOutputAmount(*amount))
    } else {
        Ok(())
    }
}

pub(crate) fn verify_output_amount_packable<const VERIFY: bool>(
    amount: &u64,
    protocol_parameters: &ProtocolParameters,
) -> Result<(), Error> {
    verify_output_amount::<VERIFY>(amount, &protocol_parameters.token_supply())
}

/// Computes the minimum amount that a storage deposit has to match to allow creating a return [`Output`] back to the
/// sender [`Address`].
fn minimum_storage_deposit(address: &Address, rent_structure: RentStructure, token_supply: u64) -> u64 {
    let address_condition = UnlockCondition::Address(AddressUnlockCondition::new(*address));
    // PANIC: This can never fail because the amount will always be within the valid range. Also, the actual value is
    // not important, we are only interested in the storage requirements of the type.
    BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
        .unwrap()
        .add_unlock_condition(address_condition)
        .finish(token_supply)
        .unwrap()
        .amount()
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize, Serializer};
    use serde_json::Value;

    use super::*;
    pub use super::{
        alias::dto::AliasOutputDto,
        alias_id::dto::AliasIdDto,
        basic::dto::BasicOutputDto,
        foundry::dto::FoundryOutputDto,
        metadata::dto::OutputMetadataDto,
        native_token::dto::NativeTokenDto,
        nft::dto::NftOutputDto,
        nft_id::dto::NftIdDto,
        rent::dto::RentStructureDto,
        token_id::dto::TokenIdDto,
        token_scheme::dto::{SimpleTokenSchemeDto, TokenSchemeDto},
        treasury::dto::TreasuryOutputDto,
    };
    use crate::block::error::dto::DtoError;

    #[derive(Clone, Debug, Deserialize, From)]
    pub enum OutputBuilderAmountDto {
        Amount(String),
        MinimumStorageDeposit(RentStructure),
    }

    /// Describes all the different output types.
    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum OutputDto {
        Treasury(TreasuryOutputDto),
        Basic(BasicOutputDto),
        Alias(AliasOutputDto),
        Foundry(FoundryOutputDto),
        Nft(NftOutputDto),
    }

    impl From<&Output> for OutputDto {
        fn from(value: &Output) -> Self {
            match value {
                Output::Treasury(o) => Self::Treasury(o.into()),
                Output::Basic(o) => Self::Basic(o.into()),
                Output::Alias(o) => Self::Alias(o.into()),
                Output::Foundry(o) => Self::Foundry(o.into()),
                Output::Nft(o) => Self::Nft(o.into()),
            }
        }
    }

    impl Output {
        pub fn try_from_dto(value: &OutputDto, token_supply: u64) -> Result<Self, DtoError> {
            Ok(match value {
                OutputDto::Treasury(o) => Self::Treasury(TreasuryOutput::try_from_dto(o, token_supply)?),
                OutputDto::Basic(o) => Self::Basic(BasicOutput::try_from_dto(o, token_supply)?),
                OutputDto::Alias(o) => Self::Alias(AliasOutput::try_from_dto(o, token_supply)?),
                OutputDto::Foundry(o) => Self::Foundry(FoundryOutput::try_from_dto(o, token_supply)?),
                OutputDto::Nft(o) => Self::Nft(NftOutput::try_from_dto(o, token_supply)?),
            })
        }

        pub fn try_from_dto_unverified(value: &OutputDto) -> Result<Self, DtoError> {
            Ok(match value {
                OutputDto::Treasury(o) => Self::Treasury(TreasuryOutput::try_from_dto_unverified(o)?),
                OutputDto::Basic(o) => Self::Basic(BasicOutput::try_from_dto_unverified(o)?),
                OutputDto::Alias(o) => Self::Alias(AliasOutput::try_from_dto_unverified(o)?),
                OutputDto::Foundry(o) => Self::Foundry(FoundryOutput::try_from_dto_unverified(o)?),
                OutputDto::Nft(o) => Self::Nft(NftOutput::try_from_dto_unverified(o)?),
            })
        }
    }

    impl<'de> Deserialize<'de> for OutputDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid output type"))? as u8
                {
                    TreasuryOutput::KIND => {
                        Self::Treasury(TreasuryOutputDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize treasury output: {e}"))
                        })?)
                    }
                    BasicOutput::KIND => Self::Basic(
                        BasicOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize basic output: {e}")))?,
                    ),
                    AliasOutput::KIND => Self::Alias(
                        AliasOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize alias output: {e}")))?,
                    ),
                    FoundryOutput::KIND => Self::Foundry(
                        FoundryOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize foundry output: {e}")))?,
                    ),
                    NftOutput::KIND => Self::Nft(
                        NftOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize NFT output: {e}")))?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid output type")),
                },
            )
        }
    }

    impl Serialize for OutputDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum OutputDto_<'a> {
                T1(&'a TreasuryOutputDto),
                T2(&'a BasicOutputDto),
                T3(&'a AliasOutputDto),
                T4(&'a FoundryOutputDto),
                T5(&'a NftOutputDto),
            }
            #[derive(Serialize)]
            struct TypedOutput<'a> {
                #[serde(flatten)]
                output: OutputDto_<'a>,
            }
            let output = match self {
                Self::Treasury(o) => TypedOutput {
                    output: OutputDto_::T1(o),
                },
                Self::Basic(o) => TypedOutput {
                    output: OutputDto_::T2(o),
                },
                Self::Alias(o) => TypedOutput {
                    output: OutputDto_::T3(o),
                },
                Self::Foundry(o) => TypedOutput {
                    output: OutputDto_::T4(o),
                },
                Self::Nft(o) => TypedOutput {
                    output: OutputDto_::T5(o),
                },
            };
            output.serialize(serializer)
        }
    }
}
