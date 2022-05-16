// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer API

pub mod query_parameters;
pub mod responses;
pub mod routes;

use std::str::FromStr;

use bee_message::output::OutputId;

use self::{
    query_parameters::{QueryParameter, QueryParameters},
    responses::OutputIdsResponse,
};
use crate::{Client, Result};

impl Client {
    /// Get all output ids for a provided URL route and query parameters
    pub async fn get_output_ids_with_pagination(
        &self,
        route: &str,
        query_parameters: Vec<QueryParameter>,
        need_quorum: bool,
        prefer_permanode: bool,
    ) -> Result<Vec<OutputId>> {
        let mut query_parameters = QueryParameters::new(query_parameters);
        let mut all_output_ids: Vec<OutputId> = Vec::new();

        while let Some(cursor) = {
            let outputs_response: OutputIdsResponse = self
                .node_manager
                .get_request(
                    route,
                    query_parameters.into_query_sting().as_deref(),
                    self.get_timeout(),
                    need_quorum,
                    prefer_permanode,
                )
                .await?;

            for output_id in outputs_response.items {
                all_output_ids.push(OutputId::from_str(&output_id)?);
            }

            outputs_response.cursor
        } {
            query_parameters.replace(QueryParameter::Cursor(cursor));
        }

        Ok(all_output_ids)
    }
}
