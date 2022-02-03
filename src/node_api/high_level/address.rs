// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{node_api::indexer_api::query_parameters::QueryParameter, Client, Result};

use bee_message::output::{Output, TokenId};
use bee_rest_api::types::responses::OutputResponse;
use primitive_types::U256;

use std::collections::{hash_map::Entry, HashMap};

/// Balance information for an address.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressBalance {
    /// bech32 encoded address
    pub address: String,
    /// IOTA balance
    pub balance: u64,
    /// native tokens
    pub native_tokens: HashMap<TokenId, U256>,
    /// The ledger index at which the outputs were retrieved
    #[serde(rename = "ledgerIndex", default)]
    pub ledger_index: u32,
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

    /// Consume the builder and get the IOTA and native tokens balance of a given Bech32 encoded address.
    pub async fn balance(self, address: &str) -> Result<AddressBalance> {
        let output_ids = crate::node_api::indexer_api::routes::output_ids(
            self.client,
            vec![QueryParameter::Address(address.to_string())],
        )
        .await?;

        let outputs_responses: Vec<OutputResponse> =
            crate::node_api::core_api::get_outputs(self.client, output_ids).await?;

        let mut total_balance = 0;
        let mut native_tokens_map = HashMap::new();
        for output_response in outputs_responses.iter() {
            let output = Output::try_from(&output_response.output)?;
            if let Some(native_tokens) = output.native_tokens() {
                for native_token in native_tokens {
                    match native_tokens_map.entry(*native_token.token_id()) {
                        Entry::Vacant(e) => {
                            e.insert(*native_token.amount());
                        }
                        Entry::Occupied(mut e) => {
                            *e.get_mut() += *native_token.amount();
                        }
                    }
                }
            }
            total_balance += output.amount();
        }
        let ledger_index = {
            if outputs_responses.is_empty() {
                0
            } else {
                outputs_responses[0].ledger_index
            }
        };
        Ok(AddressBalance {
            address: address.to_string(),
            balance: total_balance,
            ledger_index,
            native_tokens: native_tokens_map,
        })
    }
    /// Get outputs
    pub async fn outputs(self, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputResponse>> {
        let output_ids = crate::node_api::indexer_api::routes::output_ids(self.client, query_parameters).await?;
        crate::node_api::core_api::get_outputs(self.client, output_ids).await
    }
}
