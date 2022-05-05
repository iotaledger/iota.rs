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

/// Get all output ids for a provided URL route and query parameters
pub async fn get_output_ids_with_pagination(
    client: &Client,
    route: &str,
    query_parameters: Vec<QueryParameter>,
) -> Result<Vec<OutputId>> {
    let mut query_parameters = QueryParameters::new(query_parameters);
    // do we need to validate the query parameters?
    let mut all_output_ids: Vec<OutputId> = Vec::new();
    while let Some(cursor) = {
        let outputs_response: OutputIdsResponse = client
            .node_manager
            .get_request(
                route,
                query_parameters.into_query_sting().as_deref(),
                client.get_timeout(),
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
