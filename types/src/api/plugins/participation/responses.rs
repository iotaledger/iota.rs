// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Response types for the public participation endpoints.
//! Types from <https://github.com/iotaledger/inx-participation/blob/d3b994f74a8bb948b18a89b04ed6c9bb271c7166/core/participation/types.go>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    api::plugins::participation::types::ParticipationEventId,
    block::{output::OutputId, BlockId},
};

/// EventsResponse defines the response of a GET RouteParticipationEvents REST API call.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsResponse {
    /// The hex encoded ID of the created participation events.
    pub event_ids: Vec<ParticipationEventId>,
}

/// TrackedParticipation holds the information for each tracked participation.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedParticipation {
    /// ID of the block that included the transaction that created the output the participation was made.
    pub block_id: BlockId,
    /// Amount of tokens that were included in the output the participation was made.
    pub amount: u64,
    /// Milestone index the participation started.
    pub start_milestone_index: u32,
    /// Milestone index the participation ended. 0 if the participation is still active.
    pub end_milestone_index: u32,
    /// IDs of the answers to the questions of a ballot, in the same order.
    pub answers: Option<Vec<u8>>,
}

/// OutputStatusResponse defines the response of a GET RouteOutputStatus REST API call.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputStatusResponse {
    /// Participations holds the participations that were created in the output.
    pub participations: HashMap<ParticipationEventId, TrackedParticipation>,
}

/// AddressOutputsResponse defines the response of a GET RouteAddressBech32Outputs REST API call.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AddressOutputsResponse {
    /// Outputs is a map of output status per output_id.
    pub outputs: HashMap<OutputId, OutputStatusResponse>,
}
