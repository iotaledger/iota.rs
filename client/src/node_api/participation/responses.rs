// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Response types for the public participation endpoints.
//! Types from https://github.com/iotaledger/inx-participation/blob/d3b994f74a8bb948b18a89b04ed6c9bb271c7166/core/participation/types.go

use std::collections::HashMap;

use iota_types::block::{output::OutputId, BlockId};
use serde::{Deserialize, Serialize};

use crate::node_api::participation::types::EventId;

/// EventsResponse defines the response of a GET RouteParticipationEvents REST API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsResponse {
    /// The hex encoded ID of the created participation events.
    pub event_ids: Vec<EventId>,
}

/// TrackedParticipation holds the information for each tracked participation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedParticipation {
    /// Block_id is the ID of the block that included the transaction that created the output the participation was
    /// made.
    pub block_id: BlockId,
    /// Amount is the amount of tokens that were included in the output the participation was made.
    pub amount: u64,
    /// StartMilestoneIndex is the milestone index the participation started.
    pub start_milestone_index: u32,
    /// EndMilestoneIndex is the milestone index the participation ended. 0 if the participation is still active.
    pub end_milestone_index: u32,
}

/// OutputStatusResponse defines the response of a GET RouteOutputStatus REST API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputStatusResponse {
    /// Participations holds the participations that were created in the output.
    pub participations: HashMap<EventId, TrackedParticipation>,
}

/// AddressOutputsResponse defines the response of a GET RouteAddressBech32Outputs REST API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressOutputsResponse {
    /// Outputs is a map of output status per output_id.
    pub outputs: HashMap<OutputId, OutputStatusResponse>,
}
