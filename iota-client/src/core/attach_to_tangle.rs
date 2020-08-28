use crate::error::Result;
use bee_crypto::ternary::Hash;
use bee_ternary::T3B1Buf;
use bee_transaction::bundled::BundledTransaction as Transaction;

use crate::response::{AttachToTangleResponse, AttachToTangleResponseBuilder};
use crate::util::tx_trytes;
use crate::Client;

/// Builder to construct attachToTangle API
#[derive(Debug)]
pub struct AttachToTangleBuilder<'a> {
    client: &'a Client,
    trunk_transaction: String,
    branch_transaction: String,
    min_weight_magnitude: u8,
    trytes: Vec<String>,
}

impl<'a> AttachToTangleBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            trunk_transaction: Default::default(),
            branch_transaction: Default::default(),
            min_weight_magnitude: client.mwm,
            trytes: Default::default(),
        }
    }

    /// Set trunk transaction hash
    pub fn trunk_transaction(mut self, trunk_transaction: &Hash) -> Self {
        self.trunk_transaction = trunk_transaction
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>();
        self
    }

    /// Set branch transaction hash
    pub fn branch_transaction(mut self, branch_transaction: &Hash) -> Self {
        self.branch_transaction = branch_transaction
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>();
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
        let body = json!({
            "command": "attachToTangle",
            "trunkTransaction": self.trunk_transaction,
            "branchTransaction": self.branch_transaction,
            "minWeightMagnitude": self.min_weight_magnitude,
            "trytes": self.trytes,
        });

        let client = self.client;
        let res: AttachToTangleResponseBuilder = response!(client, body);
        res.build().await
    }
}
