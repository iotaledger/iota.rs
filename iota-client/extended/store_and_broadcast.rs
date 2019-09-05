use crate::client::Client;
use crate::Result;

impl<'a> Client<'a> {
    /// Helper function that both stores, and broadcast trytes to
    /// the IRI. Trytes must have been PoW-ed.
    ///
    /// * `trytes` - PoW-ed slice of tryte-encoded transaction strings
    pub fn store_and_broadcast(&mut self, trytes: &[String]) -> Result<()> {
        self.store_transactions(&trytes)?;
        self.broadcast_transactions(&trytes)?;
        Ok(())
    }
}
