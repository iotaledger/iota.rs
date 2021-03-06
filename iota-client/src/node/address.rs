// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Api, Client, Error, Result};

use bee_message::prelude::{Bech32Address, TransactionId, UTXOInput};

use bee_rest_api::endpoints::api::v1::{
    balance_ed25519::BalanceForAddressResponse, outputs_ed25519::OutputsForAddressResponse,
};

use std::convert::TryInto;

/// Output type filter.
#[derive(Clone)]
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
#[derive(Default, Clone)]
pub struct OutputsOptions {
    /// Whether the query should include spent outputs or not.
    pub include_spent: bool,
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
        match params.len() {
            0 => None,
            _ => Some(params.join("&")),
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
    pub async fn balance(self, address: &Bech32Address) -> Result<BalanceForAddressResponse> {
        let mut url = self.client.get_node().await?;
        let path = &format!("api/v1/addresses/{}", address);
        url.set_path(path);

        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: BalanceForAddressResponse,
        }
        let resp: ResponseWrapper = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetBalance))
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// Consume the builder and get all outputs that use a given address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance
    /// reasons. User should sweep the address to reduce the amount of outputs.
    pub async fn outputs(self, address: &Bech32Address, options: OutputsOptions) -> Result<Box<[UTXOInput]>> {
        let mut url = self.client.get_node().await?;
        let path = &format!("api/v1/addresses/{}/outputs", address);
        url.set_path(path);
        url.set_query(options.into_query().as_deref());

        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: OutputsForAddressResponse,
        }

        let resp: ResponseWrapper = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetOutput))
            .await?
            .json()
            .await?;

        resp.data
            .output_ids
            .iter()
            .map(|s| {
                let mut transaction_id = [0u8; 32];
                hex::decode_to_slice(&s[..64], &mut transaction_id)?;
                let index = u16::from_le_bytes(
                    hex::decode(&s[64..]).map_err(|_| Error::InvalidParameter("index"))?[..]
                        .try_into()
                        .map_err(|_| Error::InvalidParameter("index"))?,
                );
                Ok(UTXOInput::new(TransactionId::new(transaction_id), index)?)
            })
            .collect::<Result<Box<[UTXOInput]>>>()
    }
}
