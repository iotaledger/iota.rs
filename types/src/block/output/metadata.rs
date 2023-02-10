// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::{output::OutputId, payload::transaction::TransactionId, BlockId};

/// Metadata of an [`Output`](crate::block::output::Output).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputMetadata {
    /// The identifier of the block in which the output was included.
    block_id: BlockId,
    /// The identifier of the output.
    output_id: OutputId,
    /// Whether the output is spent or not.
    is_spent: bool,
    /// If spent, the index of the milestone in which the output was spent.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    milestone_index_spent: Option<u32>,
    /// If spent, the timestamp of the milestone in which the output was spent.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    milestone_timestamp_spent: Option<u32>,
    /// If spent, the identifier of the transaction that spent the output.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    transaction_id_spent: Option<TransactionId>,
    /// The index of the milestone that booked the output.
    milestone_index_booked: u32,
    /// The timestamp of the milestone that booked the output.
    milestone_timestamp_booked: u32,
    /// The index of ledger when the output was fetched.
    ledger_index: u32,
}

impl OutputMetadata {
    /// Creates a new [`OutputMetadata`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        block_id: BlockId,
        output_id: OutputId,
        is_spent: bool,
        milestone_index_spent: Option<u32>,
        milestone_timestamp_spent: Option<u32>,
        transaction_id_spent: Option<TransactionId>,
        milestone_index_booked: u32,
        milestone_timestamp_booked: u32,
        ledger_index: u32,
    ) -> Self {
        Self {
            block_id,
            output_id,
            is_spent,
            milestone_index_spent,
            milestone_timestamp_spent,
            transaction_id_spent,
            milestone_index_booked,
            milestone_timestamp_booked,
            ledger_index,
        }
    }

    /// Returns the block id of the [`OutputMetadata`].
    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }

    /// Returns the output id of the [`OutputMetadata`].
    pub fn output_id(&self) -> &OutputId {
        &self.output_id
    }

    /// Returns the transaction id of the [`OutputMetadata`].
    pub fn transaction_id(&self) -> &TransactionId {
        self.output_id.transaction_id()
    }

    /// Returns the output index of the [`OutputMetadata`].
    pub fn output_index(&self) -> u16 {
        self.output_id.index()
    }

    /// Returns whether the output is spent ot not.
    pub fn is_spent(&self) -> bool {
        self.is_spent
    }

    /// Returns the milestone index spent of the [`OutputMetadata`].
    pub fn milestone_index_spent(&self) -> Option<u32> {
        self.milestone_index_spent
    }

    /// Returns the milestone timestamp spent of the [`OutputMetadata`].
    pub fn milestone_timestamp_spent(&self) -> Option<u32> {
        self.milestone_timestamp_spent
    }

    /// Returns the transaction id spent of the [`OutputMetadata`].
    pub fn transaction_id_spent(&self) -> Option<&TransactionId> {
        self.transaction_id_spent.as_ref()
    }

    /// Returns the milestone index booked of the [`OutputMetadata`].
    pub fn milestone_index_booked(&self) -> u32 {
        self.milestone_index_booked
    }

    /// Returns the milestone timestamp booked of the [`OutputMetadata`].
    pub fn milestone_timestamp_booked(&self) -> u32 {
        self.milestone_timestamp_booked
    }

    /// Returns the ledger index of the [`OutputMetadata`].
    pub fn ledger_index(&self) -> u32 {
        self.ledger_index
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::block::error::dto::DtoError;

    /// DTO for an [`OutputMetadata`].
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputMetadataDto {
        pub block_id: String,
        pub transaction_id: String,
        pub output_index: u16,
        pub is_spent: bool,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub milestone_index_spent: Option<u32>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub milestone_timestamp_spent: Option<u32>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub transaction_id_spent: Option<String>,
        pub milestone_index_booked: u32,
        pub milestone_timestamp_booked: u32,
        pub ledger_index: u32,
    }

    impl OutputMetadataDto {
        /// Returns the output id.
        pub fn output_id(&self) -> Result<OutputId, crate::block::Error> {
            OutputId::new(TransactionId::from_str(&self.transaction_id)?, self.output_index)
        }
    }

    impl TryFrom<&OutputMetadataDto> for OutputMetadata {
        type Error = DtoError;

        fn try_from(response: &OutputMetadataDto) -> Result<Self, Self::Error> {
            Ok(Self {
                block_id: BlockId::from_str(&response.block_id)?,
                output_id: OutputId::new(
                    TransactionId::from_str(&response.transaction_id)?,
                    response.output_index,
                )?,
                is_spent: response.is_spent,
                milestone_index_spent: response.milestone_index_spent,
                milestone_timestamp_spent: response.milestone_timestamp_spent,
                transaction_id_spent: response
                    .transaction_id_spent
                    .as_ref()
                    .map(|s| TransactionId::from_str(s))
                    .transpose()?,
                milestone_index_booked: response.milestone_index_booked,
                milestone_timestamp_booked: response.milestone_timestamp_booked,
                ledger_index: response.ledger_index,
            })
        }
    }

    impl From<&OutputMetadata> for OutputMetadataDto {
        fn from(output_metadata: &OutputMetadata) -> Self {
            Self {
                block_id: output_metadata.block_id().to_string(),
                transaction_id: output_metadata.transaction_id().to_string(),
                output_index: output_metadata.output_index(),
                is_spent: output_metadata.is_spent(),
                milestone_index_spent: output_metadata.milestone_index_spent(),
                milestone_timestamp_spent: output_metadata.milestone_timestamp_spent(),
                transaction_id_spent: output_metadata.transaction_id_spent().map(|t| t.to_string()),
                milestone_index_booked: output_metadata.milestone_index_booked(),
                milestone_timestamp_booked: output_metadata.milestone_timestamp_booked(),
                ledger_index: output_metadata.ledger_index(),
            }
        }
    }
}
