use crate::error::Result;
use bee_crypto::ternary::Hash;
use bee_transaction::bundled::{Address, BundledTransactionField};
use iota_conversion::Trinary;

use crate::response::{GetBalancesResponse, GetBalancesResponseBuilder};
use crate::Client;

/// Builder to construct getBalances API
#[derive(Debug)]
pub struct GetBalancesBuilder {
    addresses: Vec<String>,
    tips: Option<Vec<String>>,
}

impl GetBalancesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            addresses: Default::default(),
            tips: Default::default(),
        }
    }

    /// Add address for which to get the balance (do not include the checksum)
    pub fn addresses(mut self, addresses: &[Address]) -> Self {
        self.addresses = addresses
            .iter()
            .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
            .collect();
        self
    }

    /// Add tips whose history of transactions to traverse to find the balance
    pub fn tips(mut self, tips: &[Hash]) -> Self {
        self.tips = Some(
            tips.iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Send getBalances request
    pub async fn send(self) -> Result<GetBalancesResponse> {
        let mut body = json!({
            "command": "getBalances",
            "addresses": self.addresses,
        });

        if let Some(reference) = self.tips {
            body["tips"] = json!(reference);
        }

        let res: GetBalancesResponseBuilder = response!(body);
        res.build().await
    }
}
