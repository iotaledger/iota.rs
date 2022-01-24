// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Indexer responses

/// Response of GET /api/plugins/indexer/v1/outputs{query parameters: "address", "requiresDustReturn", "sender", "tag"}
/// Returns the outputs for the provided query parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedOutputsResponse {
    /// The ledger index at which the outputs were collected
    #[serde(rename = "ledgerIndex")]
    pub ledger_index: u32,
    /// The max amount of output ids to be returned
    pub limit: usize,
    /// Output id for the offset
    pub offset: Option<String>,
    /// The max amount of output ids
    pub count: usize,
    /// The output ids
    pub data: Vec<String>,
}
