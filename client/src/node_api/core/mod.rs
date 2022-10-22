// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

use iota_types::{api::response::OutputResponse, block::output::OutputId};
use iota_types::api::response::OutputMetadataResponse;

#[cfg(not(target_family = "wasm"))]
use crate::constants::MAX_PARALLEL_API_REQUESTS;
use crate::{Client, Result};

impl Client {
    /// Request outputs by their output id in parallel
    pub async fn get_outputs(&self, output_ids: Vec<OutputId>) -> Result<Vec<OutputResponse>> {
        let mut outputs = Vec::new();
        #[cfg(target_family = "wasm")]
        for output_id in output_ids {
            outputs.push(self.get_output(&output_id).await?);
        }
        #[cfg(not(target_family = "wasm"))]
        for output_ids_chunk in output_ids.chunks(MAX_PARALLEL_API_REQUESTS).map(<[OutputId]>::to_vec) {
            let mut tasks = Vec::new();
            for output_id in output_ids_chunk {
                let client_ = self.clone();

                tasks.push(async move {
                    tokio::spawn(async move {
                        let output_response = client_.get_output(&output_id).await?;
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

    /// Request outputs by their output id in parallel, ignores failed requests
    /// Useful to get data about spent outputs, that might not be pruned yet
    pub async fn try_get_outputs(&self, output_ids: Vec<OutputId>) -> Result<Vec<OutputResponse>> {
        let mut outputs = Vec::new();
        #[cfg(target_family = "wasm")]
        for output_id in output_ids {
            if let Ok(output_response) = self.get_output(&output_id).await {
                outputs.push(output_response);
            }
        }
        #[cfg(not(target_family = "wasm"))]
        for output_ids_chunk in output_ids.chunks(MAX_PARALLEL_API_REQUESTS).map(<[OutputId]>::to_vec) {
            let mut tasks = Vec::new();
            for output_id in output_ids_chunk {
                let client_ = self.clone();

                tasks.push(async move {
                    tokio::spawn(async move {
                        // Ignore possible errors
                        if let Ok(output_response) = client_.get_output(&output_id).await {
                            Some(output_response)
                        } else {
                            None
                        }
                    })
                    .await
                });
            }
            for output_response in (futures::future::try_join_all(tasks).await?).into_iter().flatten() {
                outputs.push(output_response);
            }
        }
        Ok(outputs)
    }

    pub async fn try_get_metadata_for_outputs(&self, output_ids: Vec<OutputId>) -> Result<Vec<OutputMetadataResponse>> {
        let output_responses = self.try_get_outputs(output_ids).await?;
        let output_metadata_responses = output_responses.iter().map(|output_response| output_response.metadata.clone()).collect();
        Ok(output_metadata_responses)
    }
}
