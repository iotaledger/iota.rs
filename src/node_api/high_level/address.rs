// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_block::output::{NativeToken, NativeTokensBuilder, Output};

use crate::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

/// Balance information for an address.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressBalance {
    /// bech32 encoded address
    pub address: String,
    /// IOTA balance
    pub balance: u64,
    /// native tokens
    pub native_tokens: Vec<NativeToken>,
    /// The ledger index at which the outputs were retrieved
    #[serde(rename = "ledgerIndex", default)]
    pub ledger_index: u32,
}

/// Builder of GET /api/core/v2/address/{address} endpoint
#[must_use]
pub struct GetAddressBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAddressBuilder<'a> {
    /// Create GET /api/core/v2/address/{address} endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Consume the builder and get the IOTA and native tokens balance of a given Bech32 encoded address, ignoring
    /// outputs with additional unlock conditions.
    pub async fn balance(self, address: &str) -> Result<AddressBalance> {
        let output_ids = self
            .client
            .basic_output_ids(vec![
                QueryParameter::Address(address.to_string()),
                QueryParameter::HasExpiration(false),
                QueryParameter::HasTimelock(false),
                QueryParameter::HasStorageDepositReturn(false),
            ])
            .await?;

        let outputs_responses = self.client.get_outputs(output_ids).await?;

        let mut total_balance = 0;
        let mut native_tokens_builder = NativeTokensBuilder::new();

        for output_response in &outputs_responses {
            let output = Output::try_from(&output_response.output)?;

            if let Some(native_tokens) = output.native_tokens() {
                native_tokens_builder.add_native_tokens(native_tokens.clone())?;
            }
            total_balance += output.amount();
        }

        let ledger_index = {
            if outputs_responses.is_empty() {
                0
            } else {
                outputs_responses[0].metadata.ledger_index
            }
        };

        Ok(AddressBalance {
            address: address.to_string(),
            balance: total_balance,
            ledger_index,
            native_tokens: native_tokens_builder.finish_vec()?,
        })
    }
}
