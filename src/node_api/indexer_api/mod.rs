// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer API

use crate::{
    node_api::indexer_api::{
        query_parameters::{QueryParameter, QueryParameters},
        responses::ExtendedOutputsResponse,
    },
    Api, Client, Error, Result,
};

use bee_message::{output::OutputId, payload::transaction::TransactionId};

use std::convert::TryInto;

const OUTPUT_ID_LENGTH: usize = 68;
const TRANSACTION_ID_LENGTH: usize = 64;

pub mod query_parameters;
pub mod responses;
pub mod routes;

/// Get all output ids for a provided URL route and query parameters
pub async fn get_output_ids_with_pagination(
    client: &Client,
    route: &str,
    mut query_parameters: QueryParameters,
) -> Result<Vec<OutputId>> {
    // do we need to validate the query parameters?
    let mut all_output_ids: Vec<OutputId> = Vec::new();
    while let Some(offset) = {
        let outputs_response: ExtendedOutputsResponse = client
            .node_manager
            .get_request(
                route,
                query_parameters.into_query_sting().as_deref(),
                client.get_timeout(Api::GetOutput),
            )
            .await?;
        // convert string response to output ids
        let output_ids = outputs_response
            .data
            .iter()
            .map(|s| {
                if s.len() == OUTPUT_ID_LENGTH {
                    let mut transaction_id = [0u8; 32];
                    hex::decode_to_slice(&s[..TRANSACTION_ID_LENGTH], &mut transaction_id)?;
                    let index = u16::from_le_bytes(
                        hex::decode(&s[TRANSACTION_ID_LENGTH..]).map_err(|_| Error::InvalidParameter("index"))?[..]
                            .try_into()
                            .map_err(|_| Error::InvalidParameter("index"))?,
                    );
                    Ok(OutputId::new(TransactionId::new(transaction_id), index)?)
                } else {
                    Err(Error::OutputError("Invalid output length from API response"))
                }
            })
            .collect::<Result<Vec<OutputId>>>()?;
        all_output_ids.extend(output_ids.into_iter());
        outputs_response.offset
    } {
        // println!("{offset}");
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        query_parameters.replace(QueryParameter::Offset(offset));
    }

    Ok(all_output_ids)
}
