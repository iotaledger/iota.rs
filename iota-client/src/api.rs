//! Main API of iota stack. These are the recommended methods to call with the nodes.

use crate::client::Client;
use crate::core::*;
use crate::error::*;
use crate::extended::*;

use bee_crypto::ternary::Hash;
use bee_signing::ternary::seed::Seed;

impl Client {
    /// Calls PrepareTransfers and then sends off the bundle via SendTrytes.
    /// * [`seed`] - An iota seed.
    /// * [`transfers`] - Transfer addresses to send data/value to.
    /// * [`inputs`] - (Optional, but recommended) Input addresses used for signing. Use `get_inputs` to get the valid inputs yourself.
    /// * [`remainder`] - (Optional) Custom remainder address.
    /// * [`security`] - (Optional) Security level. Default is 2.
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    /// * [`reference`] - (Optional) Transaction hash from which to start the weighted random walk.
    ///
    /// [`seed`]: ../extended/struct.SendTransfersBuilder.html#method.seed
    /// [`transfers`]: ../extended/struct.SendTransfersBuilder.html#method.transfers
    /// [`inputs`]: ../extended/struct.SendTransfersBuilder.html#method.inputs
    /// [`remainder`]: ../extended/struct.SendTransfersBuilder.html#method.remainder
    /// [`security`]: ../extended/struct.SendTransfersBuilder.html#method.security
    /// [`trytes`]: ../extended/struct.SendTransfersBuilder.html#method.trytes
    /// [`depth`]: ../extended/struct.SendTransfersBuilder.html#method.depth
    /// [`min_weight_magnitude`]: ../extended/struct.SendTransfersBuilder.html#method.min_weight_magnitude
    /// [`reference`]: ../extended/struct.SendTransfersBuilder.html#method.reference
    pub fn send<'a>(&'a self, seed: Option<&'a Seed>) -> SendBuilder<'a> {
        SendBuilder::new(self, seed)
    }

    /// Finds transactions that contain the given values in their transaction fields.
    /// The parameters define the transaction fields to search for, including bundles, addresses, tags, and approvees.
    /// Using multiple transaction fields, returns transactions hashes at the intersection of those values.
    /// # Parameters
    /// * [`bundles`] - (Optional) Bundle hashes to search for
    /// * [`addresses`] - (Optional) Addresses to search for (do not include the checksum)
    /// * [`tags`] - (Optional) Tags to search for
    /// * [`approvees`] - (Optional) Child transactions to search for
    ///
    /// [`bundles`]: ../core/struct.FindTransactionsBuilder.html#method.bundles
    /// [`addresses`]: ../core/struct.FindTransactionsBuilder.html#method.addresses
    /// [`tags`]: ../core/struct.FindTransactionsBuilder.html#method.tags
    /// [`approvees`]: ../core/struct.FindTransactionsBuilder.html#method.approvees
    pub fn find_transactions(&self) -> FindTransactionsBuilder<'_> {
        FindTransactionsBuilder::new(self)
    }

    /// Generates and returns a new address by calling `find_transactions` until the first unused address is detected.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`index`] - (Optional) Key index to start search at. Default is 0.
    /// * [`security`] - (Optional) Security level. Default is 2.
    ///
    /// [`seed`]: ../extended/struct.GenerateNewAddressBuilder.html#method.seed
    /// [`index`]: ../extended/struct.GenerateNewAddressBuilder.html#method.index
    /// [`security`]: ../extended/struct.GenerateNewAddressBuilder.html#method.security
    pub fn generate_new_address<'a>(&'a self, seed: &'a Seed) -> GenerateNewAddressBuilder<'a> {
        GenerateNewAddressBuilder::new(self, seed)
    }

    // TODO get_addresses

    /// Returns the balance for a provided seed by checking the addresses for a seed up until a given point.
    pub async fn get_balance(&self, seed: &Seed) -> Result<u64> {
        Ok(GetInputsBuilder::new(self, seed).generate().await?.0)
    }

    /// Gets the confirmed balance of an address.
    /// If the tips parameter is missing, the returned balance is correct as of the latest confirmed milestone.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * [`addresses`] - Address for which to get the balance (do not include the checksum)
    /// * [`threshold`] - (Optional) Confirmation threshold between 0 and 100, default is 100.
    /// * [`tips`] - (Optional) Tips whose history of transactions to traverse to find the balance
    ///
    /// [`addresses`]: ../core/struct.GetBalancesBuilder.html#method.addresses
    /// [`threshold`]: ../core/struct.GetBalancesBuilder.html#method.threshold
    /// [`tips`]: ../core/struct.GetBalancesBuilder.html#method.tips
    pub fn get_balance_of_addresses(&self) -> GetBalancesBuilder<'_> {
        GetBalancesBuilder::new(self)
    }

    /// Reattaches a transfer to tangle by selecting tips & performing the Proof-of-Work again.
    /// Reattachments are useful in case original transactions are pending and can be done securely as many times as needed.
    /// # Parameters
    /// * `hash` - Tail transaction hash. Tail transaction is the transaction in the bundle with current_index == 0
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    ///
    /// [`depth`]: ../extended/struct.SendTrytesBuilder.html#method.depth
    /// [`min_weight_magnitude`]: ../extended/struct.SendTrytesBuilder.html#method.min_weight_magnitude
    pub async fn reattach<'a>(&'a self, hash: &'a Hash) -> Result<SendTrytesBuilder<'a>> {
        let mut bundle = self.get_bundle(hash).await?;
        bundle.reverse();
        Ok(SendTrytesBuilder::new(self).trytes(bundle))
    }

    /// Fetch inclusion states of the given transactions to determine if the transactions are confirmed.
    ///
    /// # Parameters
    /// * [`transactions`] - List of transaction hashes for which you want to get the inclusion state
    pub async fn is_confirmed(&self, transactions: &[Hash]) -> Result<Vec<bool>> {
        let states = self
            .get_inclusion_states()
            .transactions(transactions)
            .send()
            .await?
            .states;
        Ok(states)
    }
}
