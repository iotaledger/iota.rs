// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{node::ExtendedOutputsResponse, Client, Error, Result};

use crate::bee_rest_api::types::responses::OutputResponse;
use bee_message::{input::UtxoInput, output::OutputId, payload::transaction::TransactionId};
use bee_rest_api::types::{dtos::OutputDto, responses::BalanceAddressResponse};

use std::convert::TryInto;

const OUTPUT_ID_LENGTH: usize = 68;
const TRANSACTION_ID_LENGTH: usize = 64;

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

/// The outputs query options.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OutputsOptions {
    #[serde(rename = "outputType")]
    /// The bech32_address type filter.
    pub bech32_address: Option<String>,
}

impl OutputsOptions {
    fn into_query(self) -> Option<String> {
        let mut params = Vec::new();
        if let Some(bech32_address) = self.bech32_address {
            params.push(format!("address={}", bech32_address))
        }
        if params.is_empty() {
            None
        } else {
            Some(params.join("&"))
        }
    }
}

/// Builder of GET /api/v2/address/{address} endpoint
pub struct GetAddressBuilder {
    client: Client,
}

impl GetAddressBuilder {
    /// Create GET /api/v2/address/{address} endpoint builder
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    /// Consume the builder and get the balance of a given Bech32 encoded address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn balance(self, address: &str) -> Result<BalanceAddressResponse> {
        let outputs_response: Vec<OutputResponse> = self
            .client
            .get_address()
            .outputs(OutputsOptions {
                bech32_address: Some(address.to_string()),
            })
            .await?;

        let mut total_balance = 0;

        for output in outputs_response.iter() {
            let amount = match &output.output {
                OutputDto::Extended(o) => o.amount,
                _ => 0,
            };
            total_balance += amount;
        }

        Ok(BalanceAddressResponse {
            address: address.to_string(),
            // todo remove this and only use the bech32 address?
            address_type: 0,
            balance: total_balance,
            ledger_index: outputs_response[0].ledger_index,
        })
    }
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn outputs(self, options: OutputsOptions) -> Result<Vec<OutputResponse>> {
        let path = "api/plugins/indexer/v1/outputs";

        let outputs_response: ExtendedOutputsResponse = self
            .client
            .node_manager
            .get_request(path, options.into_query().as_deref(), self.client.get_timeout())
            .await?;
        // todo pagination
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
                    Ok(UtxoInput::new(TransactionId::new(transaction_id), index)?)
                } else {
                    Err(Error::OutputError("Invalid output length from API response"))
                }
            })
            .collect::<Result<Box<[UtxoInput]>>>()?;

        let mut outputs = Vec::new();

        for output_id in output_ids.iter() {
            let output = self.client.get_output(output_id.output_id()).await?;
            outputs.push(output);
        }
        Ok(outputs)
    }

    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn output_ids(self, options: OutputsOptions) -> Result<Box<[OutputId]>> {
        let path = "api/plugins/indexer/v1/outputs";

        let outputs_response: ExtendedOutputsResponse = self
            .client
            .node_manager
            .get_request(path, options.into_query().as_deref(), self.client.get_timeout())
            .await?;
        // todo pagination
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
            .collect::<Result<Box<[OutputId]>>>()?;
        Ok(output_ids)
    }
}
