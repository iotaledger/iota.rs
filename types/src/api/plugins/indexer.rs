// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node indexer responses.

use core::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::block::output::OutputId;

/// Response of GET /api/indexer/v1/*
/// Returns the output_ids for the provided query parameters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputIdsResponse {
    /// The ledger index at which the outputs were collected
    #[serde(rename = "ledgerIndex")]
    pub ledger_index: u32,
    /// Cursor confirmationMS+outputId.pageSize
    pub cursor: Option<String>,
    /// The output ids
    pub items: Vec<OutputId>,
}

impl Deref for OutputIdsResponse {
    type Target = Vec<OutputId>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
