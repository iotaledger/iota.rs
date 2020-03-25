use crate::client::Client;
use anyhow::Result;

impl Client<'_> {
    /// Helper function that both stores, and broadcast trytes to
    /// the IRI. Trytes must have been PoW-ed.
    ///
    /// * `trytes` - Transaction trytes
    pub async fn store_and_broadcast(&self, trytes: &[&str]) -> Result<()> {
        self.store_transactions(&trytes).await?;
        self.broadcast_transactions(&trytes).await?;

        Ok(())
    }
}
