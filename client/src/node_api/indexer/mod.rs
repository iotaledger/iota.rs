// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node indexer API.

pub mod query_parameters;
pub mod routes;

use iota_types::api::plugins::indexer::OutputIdsResponse;

pub(crate) use self::query_parameters::{QueryParameter, QueryParameters};
use crate::{Client, Result};

impl Client {
    /// Get all output ids for a provided URL route and query parameters.
    /// If a `QueryParameter::Cursor(_)` is provided, only a single page will be queried.
    pub async fn get_output_ids(
        &self,
        route: &str,
        mut query_parameters: QueryParameters,
        need_quorum: bool,
        prefer_permanode: bool,
    ) -> Result<OutputIdsResponse> {
        let mut merged_output_ids_response = OutputIdsResponse {
            ledger_index: 0,
            cursor: None,
            items: Vec::new(),
        };

        // Return early with only a single page if a `QueryParameter::Cursor(_)` is provided.
        let return_early = query_parameters.contains(QueryParameter::Cursor(String::new()).kind());

        while let Some(cursor) = {
            let output_ids_response = self
                .node_manager
                .get_request::<OutputIdsResponse>(
                    route,
                    query_parameters.to_query_string().as_deref(),
                    self.get_timeout(),
                    need_quorum,
                    prefer_permanode,
                )
                .await?;

            if return_early {
                return Ok(output_ids_response);
            }

            merged_output_ids_response.ledger_index = output_ids_response.ledger_index;
            merged_output_ids_response.cursor = output_ids_response.cursor;
            merged_output_ids_response.items.extend(output_ids_response.items);

            &merged_output_ids_response.cursor
        } {
            query_parameters.replace(QueryParameter::Cursor(cursor.to_string()));
        }

        Ok(merged_output_ids_response)
    }
}
