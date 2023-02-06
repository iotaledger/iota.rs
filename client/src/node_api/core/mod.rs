// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

use iota_types::{
    api::core::response::OutputWithMetadataResponse,
    block::output::{dto::OutputMetadataDto, OutputId},
};

#[cfg(not(target_family = "wasm"))]
use crate::constants::MAX_PARALLEL_API_REQUESTS;
use crate::{Client, Result};

impl Client {
    /// Request outputs by their output ID in parallel
    pub async fn get_outputs(&self, output_ids: Vec<OutputId>) -> Result<Vec<OutputWithMetadataResponse>> {
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

    /// Request outputs by their output ID in parallel, ignoring failed requests
    /// Useful to get data about spent outputs, that might not be pruned yet
    pub async fn try_get_outputs(&self, output_ids: Vec<OutputId>) -> Result<Vec<OutputWithMetadataResponse>> {
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

                tasks.push(async move { tokio::spawn(async move { client_.get_output(&output_id).await.ok() }).await });
            }
            for output_response in (futures::future::try_join_all(tasks).await?).into_iter().flatten() {
                outputs.push(output_response);
            }
        }
        Ok(outputs)
    }

    /// Requests metadata for outputs by their output ID in parallel, ignoring failed requests
    pub async fn try_get_outputs_metadata(&self, output_ids: Vec<OutputId>) -> Result<Vec<OutputMetadataDto>> {
        let mut output_metadata_responses = Vec::new();

        #[cfg(target_family = "wasm")]
        for output_id in output_ids {
            if let Ok(output_metadata_response) = self.get_output_metadata(&output_id).await {
                output_metadata_responses.push(output_metadata_response);
            }
        }

        #[cfg(not(target_family = "wasm"))]
        for output_ids_chunk in output_ids.chunks(MAX_PARALLEL_API_REQUESTS).map(<[OutputId]>::to_vec) {
            let mut tasks = Vec::new();
            for output_id in output_ids_chunk {
                let client_ = self.clone();

                tasks.push(async move {
                    tokio::spawn(async move { client_.get_output_metadata(&output_id).await.ok() }).await
                });
            }
            for output_metadata_response in (futures::future::try_join_all(tasks).await?).into_iter().flatten() {
                output_metadata_responses.push(output_metadata_response);
            }
        }

        Ok(output_metadata_responses)
    }
}
