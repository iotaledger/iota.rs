//! The Client module to connect through IRI with API usages
use crate::api::*;
use crate::builder::ClientBuilder;
use crate::error::*;
use crate::node::*;
use crate::types::*;

use bee_signing_ext::Seed;
use bee_transaction::atomic::MESSAGE_ID_LENGTH;
use bee_transaction::prelude::{Address, Message, MessageId, TransactionId};

use reqwest::{IntoUrl, Url};

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

    // TODO Implement syncing process

    // pub(crate) fn sync(&mut self) {
    //     let mut sync_list: HashMap<usize, Vec<Url>> = HashMap::new();
    //     for url in &*self.pool.read().unwrap() {
    //         if let Ok(milestone) = self.get_info(url.clone()) {
    //             let set = sync_list
    //                 .entry(milestone.latest_milestone_index)
    //                 .or_insert(Vec::new());
    //             set.push(url.clone());
    //         };
    //     }

    //     *self.sync.write().unwrap() = sync_list.into_iter().max_by_key(|(x, _)| *x).unwrap().1;
    // }

    /// Get a node candidate from the node pool.
    pub(crate) fn get_node(&self) -> Result<Url> {
        Ok(self
            .pool
            .read()
            .unwrap()
            .iter()
            .next()
            .ok_or(Error::NodePoolEmpty)?
            .clone())
    }

    //////////////////////////////////////////////////////////////////////
    // Node API
    //////////////////////////////////////////////////////////////////////

    /// GET /health endpoint
    pub async fn get_health<T: IntoUrl>(url: T) -> Result<bool> {
        let mut url = url.into_url()?;
        url.set_path("health");
        let r = reqwest::get(url).await?;

        match r.status().as_u16() {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /api/v1/info endpoint
    pub async fn get_info<T: IntoUrl>(url: T) -> Result<Response<NodeInfo>> {
        let mut url = url.into_url()?;
        url.set_path("api/v1/info");
        let r = reqwest::get(url).await?.json().await?;
        Ok(r)
    }

    /// GET /api/v1/tips endpoint
    pub async fn get_tips(&self) -> Result<(MessageId, MessageId)> {
        let mut url = self.get_node()?;
        url.set_path("api/v1/tips");
        let r = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Response<Tips>>()
            .await?
            .data;

        let mut tip1 = [0u8; MESSAGE_ID_LENGTH];
        let mut tip2 = [0u8; MESSAGE_ID_LENGTH];
        hex::decode_to_slice(r.tip1, &mut tip1)?;
        hex::decode_to_slice(r.tip2, &mut tip2)?;
        Ok((MessageId::new(tip1), MessageId::new(tip2)))
    }

    /// POST /api/v1/messages endpoint
    pub fn post_messages(&self, _messages: &Message) -> Result<MessageId> {
        Ok(MessageId::new([0; 32]))
    }

    /// GET /api/v1/message/{messageId} endpoint
    pub fn get_message<'a>(&'a self, message_id: &'a MessageId) -> GetMessageBuilder<'a> {
        GetMessageBuilder::new(self, message_id)
    }
    // 41c2cad13245da7b061cddb0b8d6ef166430f8fee6d1aafe5ad5971ea7f7c729
    /// GET /api/v1/messages endpoint
    /// Search for messages matching the index
    pub fn get_messages(&self, _index: String) -> Result<Vec<MessageId>> {
        Ok(Vec::new())
    }

    /// GET /api/v1/output/{outputId} endpoint
    /// Find an output by its transaction_id and corresponding output_index.
    pub fn get_output(
        &self,
        _transaction_id: TransactionId,
        _output_index: u16,
    ) -> Result<Vec<Output>> {
        Ok(Vec::new())
    }

    /// GET /api/v1/address/{address} endpoint
    pub fn get_address<'a>(&'a self, address: &'a Address) -> GetAddressBuilder<'a> {
        GetAddressBuilder::new(self, address)
    }

    /// GET /api/v1/mileston/{index} endpoint
    /// Get the milestone by the given index.
    pub fn get_milestone(&self, _index: u64) -> Result<MessageId> {
        Ok(MessageId::new([0; 32]))
    }

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////

    /// A generic send function for easily sending value transaction messages.
    pub fn send<'a>(&'a self, seed: &'a Seed) -> SendBuilder<'a> {
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

    /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub fn reattach(&self, id: &MessageId) -> Result<Message> {
        let message = self.get_message(id).data()?;
        self.post_messages(&message)?;
        Ok(message)
    }

    /// Check if a transaction-message is confirmed.
    /// Should GET `/transaction-messages/is-confirmed`
    pub fn is_confirmed<'a>(
        &self,
        hashes: &'a [MessageId],
    ) -> Result<HashMap<&'a MessageId, bool>> {
        let mut map = HashMap::new();
        for hash in hashes {
            map.insert(hash, true);
        }
        Ok(map)
    }
}
