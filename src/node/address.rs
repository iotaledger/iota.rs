// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Api, Client, Error, Result};

use bee_message::prelude::{TransactionId, UtxoInput};

use bee_rest_api::types::{
    body::SuccessBody,
    responses::{BalanceAddressResponse, OutputsAddressResponse},
};

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
    /// Whether the query should include spent outputs or not.
    #[serde(rename = "includeSpent")]
    pub include_spent: bool,
    #[serde(rename = "outputType")]
    /// The output type filter.
    pub output_type: Option<OutputType>,
}

impl OutputsOptions {
    fn into_query(self) -> Option<String> {
        let mut params = Vec::new();
        if self.include_spent {
            params.push("include-spent=true".to_string());
        }
        if let Some(output_type) = self.output_type {
            params.push(format!("type={}", u16::from(output_type)))
        }
        if params.is_empty() {
            None
        } else {
            Some(params.join("&"))
        }
    }
}

/// Builder of GET /api/v1/address/{address} endpoint
pub struct GetAddressBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAddressBuilder<'a> {
    /// Create GET /api/v1/address/{address} endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Consume the builder and get the balance of a given Bech32 encoded address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn balance(self, address: &str) -> Result<BalanceAddressResponse> {
        let path = &format!("api/v1/addresses/{address}");

        let resp: SuccessBody<BalanceAddressResponse> = self
            .client
            .node_manager
            .get_request(path, None, self.client.get_timeout(Api::GetBalance))
            .await?;

        Ok(resp.data)
    }

    /// Consume the builder and get all outputs that use a given address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn outputs(self, address: &str, options: OutputsOptions) -> Result<Box<[UtxoInput]>> {
        let path = format!("api/v1/addresses/{address}/outputs");

        let resp: SuccessBody<OutputsAddressResponse> = self
            .client
            .node_manager
            .get_request(
                &path,
                options.into_query().as_deref(),
                self.client.get_timeout(Api::GetOutput),
            )
            .await?;

        resp.data
            .output_ids
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
            .collect::<Result<Box<[UtxoInput]>>>()
    }

    /// Consume the builder and get the OutputsAddressResponse for a given address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn outputs_response(self, address: &str, options: OutputsOptions) -> Result<OutputsAddressResponse> {
        let path = format!("api/v1/addresses/{address}/outputs");

        let resp: SuccessBody<OutputsAddressResponse> = self
            .client
            .node_manager
            .get_request(
                &path,
                options.into_query().as_deref(),
                self.client.get_timeout(Api::GetOutput),
            )
            .await?;

        Ok(resp.data)
    }
}
