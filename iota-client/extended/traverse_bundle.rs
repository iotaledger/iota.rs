use iota_model::Transaction;

use crate::client::Client;
use crate::Result;

impl<'a> Client<'a> {
    /// Traverses a bundle by going through trunk transactions until
    /// the bundle hash of the transaction is no longer the same.
    ///
    /// * `trunk_tx` - The trunk transaction to start searching at
    /// * `bundle_hash` - The bundle hash to compare against while searching
    /// * `bundle` - The bundle add transactions to, until hash no longer matches
    pub fn traverse_bundle<S, T>(
        &mut self,
        trunk_tx: &str,
        bundle_hash: S,
        bundle: T,
    ) -> Result<Vec<Transaction>>
    where
        S: Into<Option<String>>,
        T: Into<Vec<Transaction>>,
    {
        let mut bundle = bundle.into();
        let tryte_list = self
            .get_trytes(&[trunk_tx.into()])?
            .take_trytes()
            .unwrap_or_default();
        ensure!(!tryte_list.is_empty(), "Bundle transactions not visible");
        let trytes = &tryte_list[0];
        let tx: Transaction = trytes.parse()?;
        let tx_bundle = &tx.bundle;
        ensure!(tx.current_index == 0, "Invalid tail transaction supplied.");
        let bundle_hash = bundle_hash.into().unwrap_or_else(|| tx_bundle.clone());
        if bundle_hash != *tx_bundle {
            return Ok(bundle);
        }

        if tx.last_index == 0 && tx.current_index == 0 {
            return Ok(vec![tx]);
        }

        let trunk_tx = &tx.trunk_transaction;
        bundle.push(tx.clone());
        self.traverse_bundle(&trunk_tx, Some(bundle_hash), bundle)
    }
}
