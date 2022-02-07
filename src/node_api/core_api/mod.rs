// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

use crate::{node_api::core_api::routes::get_output, Client, Result};

use bee_message::output::OutputId;
use bee_rest_api::types::responses::OutputResponse;

#[cfg(not(feature = "wasm"))]
use crate::constants::MAX_PARALLEL_API_REQUESTS;

pub mod routes;

/// Request outputs by their output id in parallel
pub async fn get_outputs(client: &Client, output_ids: Vec<OutputId>) -> Result<Vec<OutputResponse>> {
    let mut outputs = Vec::new();
    #[cfg(feature = "wasm")]
    for output_id in output_ids {
        outputs.push(get_output(client, &output_id).await?);
    }
    #[cfg(not(feature = "wasm"))]
    for output_ids_chunk in output_ids
        .chunks(MAX_PARALLEL_API_REQUESTS)
        .map(|x: &[OutputId]| x.to_vec())
    {
        let mut tasks = Vec::new();
        for output_id in output_ids_chunk {
            let client_ = client.clone();

            tasks.push(async move {
                tokio::spawn(async move {
                    let output_response = get_output(&client_, &output_id).await?;
                    crate::Result::Ok(output_response)
                })
                .await
            });
        }
        for res in futures::future::try_join_all(tasks).await? {
            let output_response = res?;
            outputs.push(output_response);
        }
    }
    Ok(outputs)
}
