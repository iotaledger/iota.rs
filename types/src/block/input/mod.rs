// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod treasury;
mod utxo;

use core::ops::RangeInclusive;

use derive_more::From;

pub use self::{treasury::TreasuryInput, utxo::UtxoInput};
use crate::block::Error;

/// The maximum number of inputs of a transaction.
pub const INPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of inputs of a transaction.
pub const INPUT_COUNT_RANGE: RangeInclusive<u16> = 1..=INPUT_COUNT_MAX; // [1..128]
/// The maximum index of inputs of a transaction.
pub const INPUT_INDEX_MAX: u16 = INPUT_COUNT_MAX - 1; // 127
/// The range of valid indices of inputs of a transaction.
pub const INPUT_INDEX_RANGE: RangeInclusive<u16> = 0..=INPUT_INDEX_MAX; // [0..127]

/// A generic input supporting different input kinds.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidInputKind)]
pub enum Input {
    /// A UTXO input.
    #[packable(tag = UtxoInput::KIND)]
    Utxo(UtxoInput),
    /// A treasury input.
    #[packable(tag = TreasuryInput::KIND)]
    Treasury(TreasuryInput),
}

impl Input {
    /// Returns the input kind of an `Input`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Utxo(_) => UtxoInput::KIND,
            Self::Treasury(_) => TreasuryInput::KIND,
        }
    }

    /// Checks whether the input is a [`UtxoInput`].
    pub fn is_utxo(&self) -> bool {
        matches!(self, Self::Utxo(_))
    }

    /// Gets the input as an actual [`UtxoInput`].
    /// PANIC: do not call on a non-utxo input.
    pub fn as_utxo(&self) -> &UtxoInput {
        if let Self::Utxo(input) = self {
            input
        } else {
            panic!("as_utxo called on a non-utxo input");
        }
    }

    /// Checks whether the input is a [`TreasuryInput`].
    pub fn is_treasury(&self) -> bool {
        matches!(self, Self::Treasury(_))
    }

    /// Gets the input as an actual [`TreasuryInput`].
    /// PANIC: do not call on a non-treasury input.
    pub fn as_treasury(&self) -> &TreasuryInput {
        if let Self::Treasury(input) = self {
            input
        } else {
            panic!("as_treasury called on a non-treasury input");
        }
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    pub use super::{treasury::dto::TreasuryInputDto, utxo::dto::UtxoInputDto};
    use crate::block::error::dto::DtoError;

    /// Describes all the different input types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum InputDto {
        Utxo(UtxoInputDto),
        Treasury(TreasuryInputDto),
    }

    impl From<&Input> for InputDto {
        fn from(value: &Input) -> Self {
            match value {
                Input::Utxo(u) => Self::Utxo(u.into()),
                Input::Treasury(t) => Self::Treasury(t.into()),
            }
        }
    }

    impl TryFrom<&InputDto> for Input {
        type Error = DtoError;

        fn try_from(value: &InputDto) -> Result<Self, Self::Error> {
            match value {
                InputDto::Utxo(u) => Ok(Self::Utxo(u.try_into()?)),
                InputDto::Treasury(t) => Ok(Self::Treasury(t.try_into()?)),
            }
        }
    }
}
