// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

use crate::{node_api::core_api::routes::get_output, Client, Result};

use bee_message::output::OutputId;
use bee_rest_api::types::responses::OutputResponse;

use tokio;

pub mod routes;

// todo change
const MAX_PARALLEL_REQUESTS: usize = 5;

/// Request outputs by their output id in parallel
pub async fn get_outputs(client: Client, output_ids: Vec<OutputId>) -> Result<Vec<OutputResponse>> {
    let mut outputs = Vec::new();
    // todo single thread for wasm
    for output_ids_chunk in output_ids
        .chunks(MAX_PARALLEL_REQUESTS)
        .map(|x: &[OutputId]| x.to_vec())
    {
        let mut tasks = Vec::new();
        for (_index, output_id) in output_ids_chunk.into_iter().enumerate() {
            let client_ = client.clone();

            tasks.push(async move {
                tokio::spawn(async move {
                    // println!("push {_index}");
                    let output_response = get_output(&client_, &output_id).await?;
                    // println!("got result {_index}");
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
