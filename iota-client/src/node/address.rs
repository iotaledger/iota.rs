use crate::{AddressBalance, AddressOutputs, Client, Error, OutputIdHex, Response, Result};

use bee_message::prelude::Address;

/// Builder of GET /api/v1/address/{messageId} endpoint
pub struct GetAddressBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAddressBuilder<'a> {
    /// Create GET /api/v1/address/{messageId} endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Consume the builder and get the balance of a given address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance reasons.
    /// User should sweep the address to reduce the amount of outputs.
    pub async fn balance(self, address: &'a Address) -> Result<u64> {
        let address = match address {
            Address::Ed25519(a) => a.to_bech32(),
            _ => return Err(Error::InvalidParameter("address".to_string())),
        };
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/addresses/{}", address));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let r = resp.json::<Response<AddressBalance>>().await?.data;
                Ok(r.balance)
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// Consume the builder and get all outputs that use a given address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance reasons.
    /// User should sweep the address to reduce the amount of outputs.
    pub async fn outputs(self, address: &'a Address) -> Result<Box<[OutputIdHex]>> {
        let address = match address {
            Address::Ed25519(a) => a.to_bech32(),
            _ => return Err(Error::InvalidParameter("address".to_string())),
        };
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/addresses/{}/outputs", address));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let r = resp
                    .json::<Response<AddressOutputs>>()
                    .await?
                    .data
                    .output_ids;
                Ok(r)
            }
            status => Err(Error::ResponseError(status)),
        }
    }
}
