use crate::error::*;
use bee_crypto::ternary::{
    sponge::{Kerl, Sponge},
    Hash,
};
use bee_ternary::{T1B1Buf, TritBuf};
use bee_transaction::bundled::{BundledTransaction as Transaction, BundledTransactionField};

use crate::Client;

/// Builder to construct sendTrytes API
//#[derive(Debug)]
pub struct SendTrytesBuilder<'a> {
    client: &'a Client,
    trytes: Vec<Transaction>,
    depth: u8,
    min_weight_magnitude: u8,
    reference: Option<Hash>,
}

impl<'a> SendTrytesBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            trytes: Default::default(),
            depth: Default::default(),
            min_weight_magnitude: client.mwm,
            reference: Default::default(),
        }
    }

    /// The depth of the random walk for GTTA
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
        let mut trunk = res.trunk_transaction;
        let mut trytes = Vec::new();
        for tx in self.trytes {
            let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
            tx.as_trits_allocated(trits.as_slice_mut());
            trits.subslice_mut(7290..7533).copy_from(trunk.as_trits());
            trits
                .subslice_mut(7533..7776)
                .copy_from(res.branch_transaction.as_trits());
            let t: TritBuf<T1B1Buf> = Kerl::default().digest(&trits).unwrap();
            trunk = Hash::try_from_inner(t).map_err(|_| Error::TernaryError)?;
            trytes.push(
                Transaction::from_trits(&trits).expect("Fail to convert trits to transaction"),
            );
        }

        let res = self
            .client
            .attach_to_tangle()
            .trytes(&trytes)
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
