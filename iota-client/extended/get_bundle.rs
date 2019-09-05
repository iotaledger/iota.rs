use iota_model::Transaction;

use crate::client::Client;
use crate::Result;

impl Client<'_> {
    /// Gets the associated bundle transactions of a transaction
    /// Validates the signatures, total sum, and bundle order
    ///
    /// * `transaction` - The transaction hash to search for
    pub fn get_bundle(&mut self, transaction: &str) -> Result<Vec<Transaction>> {
        ensure!(
            iota_validation::is_hash(&transaction),
            "Invalid transaction."
        );
        let bundle = self.traverse_bundle(&transaction, None, vec![])?;
        ensure!(
            iota_validation::is_bundle(&bundle)?,
            "Invalid bundle provided."
        );
        Ok(bundle)
    }
}
