//! The Client module to connect through IRI with API usages
use crate::api::*;
use crate::builder::ClientBuilder;
use crate::error::*;
use crate::node::*;
use crate::types::*;

use bee_signing_ext::Seed;
use bee_transaction::prelude::{Address, Hash, Message};

use reqwest::Url;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

// macro_rules! response {
//     ($self:ident, $body:ident) => {
//         $self
//             .client
//             .post($self.get_node()?)
//             .header("Content-Type", "application/json")
//             .header("X-IOTA-API-Version", "1")
//             .body($body.to_string())
//             .send()
//             .await?
//             .json()
//             .await?
//     };
//     ($self:ident, $body:ident, $node:ident) => {
//         $self
//             .client
//             .post($node)
//             .header("Content-Type", "application/json")
//             .header("X-IOTA-API-Version", "1")
//             .body($body.to_string())
//             .send()
//             .await?
//             .json()
//             .await?
//     };
// }

/// An instance of the client using IRI URI
#[derive(Debug, Clone)]
pub struct Client {
    /// Node pool of IOTA nodes
    pub(crate) pool: Arc<RwLock<HashSet<Url>>>,
    pub(crate) sync: Arc<RwLock<Vec<Url>>>,
    /// A reqwest Client to make Requests with
    pub(crate) client: reqwest::Client,
    pub(crate) mwm: u8,
    pub(crate) quorum_size: u8,
    pub(crate) quorum_threshold: u8,
}

impl Client {
    /// Create the builder to instntiate the IOTA Client.
    pub fn new() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub(crate) fn sync(&mut self) {
        let mut sync_list: HashMap<usize, Vec<Url>> = HashMap::new();
        for url in &*self.pool.read().unwrap() {
            if let Ok(milestone) = self.get_info(url.clone()) {
                let set = sync_list
                    .entry(milestone.latest_solid_subtangle_milestone_index)
                    .or_insert(Vec::new());
                set.push(url.clone());
            };
        }

        *self.sync.write().unwrap() = sync_list.into_iter().max_by_key(|(x, _)| *x).unwrap().1;
    }

    /// Add a node to the node pool.
    pub fn add_node(&mut self, uri: &str) -> Result<bool> {
        let url = Url::parse(uri).map_err(|_| Error::UrlError)?;
        Ok(self.pool.write().unwrap().insert(url))
    }

    /// Remove a node from the node pool.
    pub fn remove_node(&mut self, uri: &str) -> Result<bool> {
        let url = Url::parse(uri).map_err(|_| Error::UrlError)?;
        Ok(self.pool.write().unwrap().remove(&url))
    }

    // pub(crate) fn get_node(&self) -> Result<Url> {
    //     // TODO getbalance, isconfirmed and were_addresses_spent_from should do quorum mode
    //     Ok(self
    //         .sync
    //         .read()
    //         .unwrap()
    //         .iter()
    //         .next()
    //         .ok_or(Error::NodePoolEmpty)?
    //         .clone())
    // }

    //////////////////////////////////////////////////////////////////////
    // Node API
    //////////////////////////////////////////////////////////////////////

    /// GET /info endpoint
    pub fn get_info(&self, _url: Url) -> Result<NodeInfo> {
        Ok(NodeInfo {
            name: String::from("Bee"),
            version: String::from("v0.1.0"),
            is_healthy: true,
            latest_solid_subtangle_milestone_index: 0,
        })
    }

    /// GET /tips endpoint
    pub fn get_tips(&self) -> Result<(Hash, Hash)> {
        Ok((Hash::new([0; 32]), Hash::new([0; 32])))
    }

    /// GET /messages/* endpoint
    pub fn get_messages(&self) -> GetMessagesBuilder<'_> {
        GetMessagesBuilder::new(self)
    }

    /// POST /messages endpoint
    pub fn post_messages(&self, _messages: &[Message]) -> Result<Vec<Hash>> {
        Ok(Vec::new())
    }

    /// GET /transaction-messages/* endpoint
    pub fn get_transactions(&self) -> GetTransactionsBuilder<'_> {
        GetTransactionsBuilder::new(self)
    }

    /// GET /outputs/* endpoint
    pub fn get_outputs(&self) -> GetOutputsBuilder<'_> {
        GetOutputsBuilder::new(self)
    }

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////

    /// A generic send function for easily sending value transaction messages.
    pub fn sned<'a>(&'a self, seed: &'a Seed) -> SendBuilder<'a> {
        SendBuilder::new(self, seed)
    }
    /// Return a valid unuspent address.
    pub fn get_unspent_address<'a>(&'a self, seed: &'a Seed) -> GetUnspentAddressBuilder<'a> {
        GetUnspentAddressBuilder::new(self, seed)
    }

    /// Return a list of addresses from the seed regardless of their validity.
    pub fn get_addresses<'a>(&'a self, seed: &'a Seed) -> GetAddressesBuilder<'a> {
        GetAddressesBuilder::new(self, seed)
    }

    /// Return the balance for a provided seed and its wallet chain BIP32 path. BIP32 derivation path of the address should be in form of `m/0'/0'/k'`. So the wallet chain is expected to be `m/0'/0'`. Addresses with balance must be consecutive, so this method will return once it encounters a zero balance address.
    pub fn get_balance<'a>(&'a self, seed: &'a Seed) -> GetBalanceBuilder<'a> {
        GetBalanceBuilder::new(self, seed)
    }

    /// Returns the balance in iota for the given addresses; No seed or security level
    /// needed to do this since we are only checking and already know the addresses.
    /// For convinience, it returns a vector of `Output` so users can get more contexts about
    /// addresses.
    pub fn get_addresses_balance(&self, addresses: &[Address]) -> Result<Vec<Output>> {
        self.get_outputs().addresses(addresses).get()
    }

    /// Reattaches messages for provided message hashes. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub fn reattach(&self, hashes: &[Hash]) -> Result<Vec<Message>> {
        let messages = self.get_messages().hashes(hashes).get()?;
        self.post_messages(&messages)?;
        Ok(messages)
    }

    /// Check if a transaction-message is confirmed.
    /// Should GET `/transaction-messages/is-confirmed`
    pub fn is_confirmed<'a>(&self, hashes: &'a [Hash]) -> Result<Vec<bool>> {
        let mut states = vec![];
        for _ in hashes {
            states.push(true);
        }
        Ok(states)
    }
}
