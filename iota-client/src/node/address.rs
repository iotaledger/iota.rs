use crate::{Client, Result, Output};

use bee_transaction::prelude::Address;

/// Builder of GET /api/v1/address/{messageId} endpoint
pub struct GetAddressBuilder<'a> {
    _client: &'a Client,
    address: &'a Address,
}

impl<'a> GetAddressBuilder<'a> {
    /// Create GET /api/v1/address/{messageId} endpoint builder
    pub fn new(_client: &'a Client, address: &'a Address) -> Self {
        Self {
            _client,
            address,
        }
    }

    /// Consume the builder and get the balance of a given address.
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance reasons.
    /// User should sweep the address to reduce the amount of outputs.
    pub fn balance(self) -> Result<u64> {
        Ok(1)
    }

    /// Consume the builder and get all outputs that use a given address. 
    /// If count equals maxResults, then there might be more outputs available but those were skipped for performance reasons.
    /// User should sweep the address to reduce the amount of outputs.
    pub fn outputs(self) -> Result<Vec<Output>> {
        Ok(Vec::new())
    }
}
