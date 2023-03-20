// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling for input selection.

use std::fmt::{Debug, Display};

use iota_types::block::output::{ChainId, OutputId, TokenId};
use primitive_types::U256;
use serde::{Serialize, Serializer};

use crate::api::input_selection::Requirement;

/// Errors related to input selection.
#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "error", rename_all = "camelCase")]
pub enum Error {
    /// Block error.
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Block(#[from] iota_types::block::Error),
    /// Can't burn and transition an output at the same time.
    #[error("can't burn and transition an output at the same time, chain ID: {0}")]
    BurnAndTransition(ChainId),
    /// Insufficient amount provided.
    #[error("insufficient amount: found {found}, required {required}")]
    InsufficientAmount {
        /// The amount found.
        found: u64,
        /// The required amount.
        required: u64,
    },
    /// Insufficient native token amount provided.
    #[error("insufficient native token amount: found {found}, required {required}")]
    InsufficientNativeTokenAmount {
        /// The token ID.
        token_id: TokenId,
        /// The amount found.
        found: U256,
        /// The required amount.
        required: U256,
    },
    /// No input with matching ed25519 address provided.
    #[error("no input with matching ed25519 address provided")]
    MissingInputWithEd25519Address,
    /// No available inputs were provided to input selection.
    #[error("no available inputs provided")]
    NoAvailableInputsProvided,
    /// No outputs were provided to input selection.
    #[error("no outputs provided")]
    NoOutputsProvided,
    /// Required input is forbidden.
    #[error("required input {0} is forbidden")]
    RequiredInputIsForbidden(OutputId),
    /// Required input is not available.
    #[error("required input {0} is not available")]
    RequiredInputIsNotAvailable(OutputId),
    /// Unfulfillable requirement.
    #[error("unfulfillable requirement {0:?}")]
    UnfulfillableRequirement(Requirement),
}

/// Use this to serialize Error variants that implements Debug but not Serialize
pub(crate) fn display_string<T, S>(value: &T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    value.to_string().serialize(serializer)
}
