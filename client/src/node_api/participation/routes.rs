// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node public participation routes.
//! https://github.com/iota-community/treasury/blob/main/specifications/hornet-participation-plugin.md#public-node-endpoints
//! https://github.com/iotaledger/inx-participation/blob/develop/core/participation/routes.go

use iota_types::block::output::OutputId;

use super::{
    responses::{AddressOutputsResponse, EventsResponse, OutputStatusResponse},
    types::{AddressStakingStatus, EventData, EventId, EventStatus, ParticipationEventType},
};
use crate::{Client, Result};

impl Client {
    /// RouteParticipationEvents is the route to list all events, returning their ID, the event name and status.
    pub async fn events(&self, event_type: Option<ParticipationEventType>) -> Result<EventsResponse> {
        let route = "api/participation/v1/events";

        let query = if let Some(event_type) = event_type {
            let query_string = match event_type {
                ParticipationEventType::Voting => "0",
                ParticipationEventType::Staking => "1",
            };
            Some(query_string)
        } else {
            None
        };

        self.node_manager
            .get_request(route, query, self.get_timeout(), false, false)
            .await
    }

    /// RouteParticipationEvent is the route to access a single participation by its ID.
    pub async fn event(&self, event_id: &EventId) -> Result<EventData> {
        let route = format!("api/participation/v1/events/{event_id}");

        self.node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }

    /// RouteParticipationEventStatus is the route to access the status of a single participation by its ID.
    pub async fn event_status(&self, event_id: &EventId, milestone_index: Option<u32>) -> Result<EventStatus> {
        let route = format!("api/participation/v1/events/{event_id}/status");

        self.node_manager
            .get_request(
                &route,
                milestone_index.map(|index| index.to_string()).as_deref(),
                self.get_timeout(),
                false,
                false,
            )
            .await
    }

    /// RouteOutputStatus is the route to get the vote status for a given output ID.
    pub async fn output_status(&self, output_id: &OutputId) -> Result<OutputStatusResponse> {
        let route = format!("api/participation/v1/outputs/{output_id}");

        self.node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }

    /// RouteAddressBech32Status is the route to get the staking rewards for the given bech32 address.
    pub async fn address_staking_status(&self, bech32_address: &str) -> Result<AddressStakingStatus> {
        let route = format!("api/participation/v1/addresses/{bech32_address}");

        self.node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }

    /// RouteAddressBech32Outputs is the route to get the outputs for the given bech32 address.
    pub async fn address_participation_output_ids(&self, bech32_address: &str) -> Result<AddressOutputsResponse> {
        let route = format!("api/participation/v1/addresses/{bech32_address}/outputs");

        self.node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }
}
