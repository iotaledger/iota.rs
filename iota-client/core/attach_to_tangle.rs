use anyhow::Result;
use iota_bundle_preview::{Hash, Transaction};
use iota_conversion::Trinary;

use crate::response::{AttachToTangleResponse, AttachToTangleResponseBuilder};
use crate::util::tx_trytes;
use crate::Client;

/// Builder to construct attachToTangle API
#[derive(Debug)]
pub struct AttachToTangleBuilder {
    trunk_transaction: String,
    branch_transaction: String,
    min_weight_magnitude: u8,
    trytes: Vec<String>,
}

impl AttachToTangleBuilder {
    pub(crate) fn new() -> Self {
        Self {
            trunk_transaction: Default::default(),
            branch_transaction: Default::default(),
            min_weight_magnitude: 14,
            trytes: Default::default(),
        }
    }

    /// Set trunk transaction hash
    pub fn trunk_transaction(mut self, trunk_transaction: &Hash) -> Self {
        self.trunk_transaction = trunk_transaction.as_bytes().trytes().unwrap();
        self
    }

    /// Set branch transaction hash
    pub fn branch_transaction(mut self, branch_transaction: &Hash) -> Self {
        self.branch_transaction = branch_transaction.as_bytes().trytes().unwrap();
        self
    }

    /// Set difficulty of PoW
    pub fn min_weight_magnitude(mut self, min_weight_magnitude: u8) -> Self {
        self.min_weight_magnitude = min_weight_magnitude;
        self
    }

    /// Add slice of transaction trytes. When sending transactions in a bundle,
    /// make sure that the trytes of the last transaction in the bundle are in index 0 of the array.
    pub fn trytes(mut self, trytes: &[Transaction]) -> Self {
        self.trytes = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        self
    }

    /// Send attachToTangle request
    pub async fn send(self) -> Result<AttachToTangleResponse> {
        let client = Client::get();
        let body = json!({
            "command": "attachToTangle",
            "trunkTransaction": self.trunk_transaction,
            "branchTransaction": self.branch_transaction,
            "minWeightMagnitude": self.min_weight_magnitude,
            "trytes": self.trytes,
        });

        let res: AttachToTangleResponseBuilder = response!(client, body);
        res.build().await
    }
}
