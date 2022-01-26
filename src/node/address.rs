// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{node_api::indexer_api::query_parameters::QueryParameter, Client, Result};

use crate::bee_rest_api::types::responses::OutputResponse;
use bee_rest_api::types::{dtos::OutputDto, responses::BalanceAddressResponse};

/// Output type filter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutputType {
    /// Signature locked single output.
    SignatureLockedSingle,
    /// Dust allowance output.
    SignatureLockedDustAllowance,
}

impl From<OutputType> for u16 {
    fn from(value: OutputType) -> Self {
        match value {
            OutputType::SignatureLockedSingle => 0,
            OutputType::SignatureLockedDustAllowance => 1,
        }
    }
}

/// Builder of GET /api/v2/address/{address} endpoint
pub struct GetAddressBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAddressBuilder<'a> {
    /// Create GET /api/v2/address/{address} endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Consume the builder and get the balance of a given Bech32 encoded address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn balance(self, address: &str) -> Result<BalanceAddressResponse> {
        let output_ids = crate::node_api::indexer_api::routes::output_ids(
            self.client,
            vec![QueryParameter::Address(address.to_string())],
        )
        .await?;

        let outputs_response: Vec<OutputResponse> =
            crate::node_api::core_api::get_outputs(self.client, output_ids).await?;

        let mut total_balance = 0;

        for output in outputs_response.iter() {
            let amount = match &output.output {
                OutputDto::Extended(o) => o.amount,
                _ => 0,
            };
            total_balance += amount;
        }
        let ledger_index = {
            if outputs_response.is_empty() {
                0
            } else {
                outputs_response[0].ledger_index
            }
        };
        Ok(BalanceAddressResponse {
            address: address.to_string(),
            // todo remove this and only use the bech32 address?
            address_type: 0,
            balance: total_balance,
            ledger_index,
        })
    }
    /// Get outputs
    pub async fn outputs(self, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputResponse>> {
        let output_ids = crate::node_api::indexer_api::routes::output_ids(self.client, query_parameters).await?;
        crate::node_api::core_api::get_outputs(self.client, output_ids).await
    }
}
