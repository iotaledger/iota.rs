use anyhow::Result;
use bee_bundle::{Hash, Transaction};

use crate::Client;

/// Builder to construct sendTrytes API
//#[derive(Debug)]
pub struct SendTrytesBuilder<'a> {
    client: &'a Client<'a>,
    trytes: Vec<Transaction>,
    depth: u8,
    min_weight_magnitude: u8,
    reference: Option<Hash>,
}

impl<'a> SendTrytesBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            trytes: Default::default(),
            depth: Default::default(),
            min_weight_magnitude: Default::default(),
            reference: Default::default(),
        }
    }

    /// Set MWM depth
    pub fn depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }

    /// Set difficulty of PoW
    pub fn min_weight_magnitude(mut self, min_weight_magnitude: u8) -> Self {
        self.min_weight_magnitude = min_weight_magnitude;
        self
    }

    /// Add vector of transaction trytes
    pub fn trytes(mut self, trytes: Vec<Transaction>) -> Self {
        self.trytes = trytes;
        self
    }

    /// Add reference hash
    pub fn reference(mut self, reference: Hash) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Send SendTrytes request
    pub async fn send(self) -> Result<Vec<Transaction>> {
        let mut gtta = self.client.get_transactions_to_approve().depth(self.depth);
        if let Some(hash) = self.reference {
            gtta = gtta.reference(&hash);
        }
        let res = gtta.send().await?;

        let res = self
            .client
            .attach_to_tangle()
            .trytes(&self.trytes)
            .branch_transaction(&res.branch_transaction)
            .trunk_transaction(&res.trunk_transaction)
            .min_weight_magnitude(self.min_weight_magnitude)
            .send()
            .await?
            .trytes;

        self.client.store_and_broadcast(&res).await?;

        Ok(res)
    }
}
