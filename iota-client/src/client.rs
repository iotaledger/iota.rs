//! The Client module to connect through IRI with API usages
use crate::api::*;
use crate::builder::ClientBuilder;
use crate::error::*;
use crate::node::*;
use crate::types::*;

use bee_message::prelude::{Address, Ed25519Address, Message, MessageId, UTXOInput};
use bee_signing_ext::Seed;

use reqwest::{IntoUrl, Url};

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

const ADDRESS_LENGTH: usize = 32;

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
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /api/v1/info endpoint
    pub async fn get_info<T: IntoUrl>(url: T) -> Result<Response<NodeInfo>> {
        let mut url = url.into_url()?;
        url.set_path("api/v1/info");
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => Ok(resp.json().await?),
            status => Err(Error::ResponseError(status)),
        }
    }

    /// GET /api/v1/tips endpoint
    pub async fn get_tips(&self) -> Result<(MessageId, MessageId)> {
        let mut url = self.get_node()?;
        url.set_path("api/v1/tips");
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let pair = resp.json::<Response<Tips>>().await?.data;
                let (mut tip1, mut tip2) = ([0u8; 32], [0u8; 32]);
                hex::decode_to_slice(pair.tip1, &mut tip1)?;
                hex::decode_to_slice(pair.tip2, &mut tip2)?;

                Ok((MessageId::from(tip1), MessageId::from(tip2)))
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// POST /api/v1/messages endpoint
    pub async fn post_message(&self, message: &Message) -> Result<MessageId> {
        let mut url = self.get_node()?;
        url.set_path("api/v1/messages");
        let message: MessageJson = message.into();
        let resp = self
            .client
            .post(url)
            .header("content-type", "application/json; charset=UTF-8")
            .json(&message)
            .send()
            .await?;

        match resp.status().as_u16() {
            201 => {
                let m = resp.json::<Response<PostMessageId>>().await?.data;
                let mut message_id = [0u8; 32];
                hex::decode_to_slice(m.message_id, &mut message_id)?;
                Ok(MessageId::from(message_id))
            }
            status => {
                println!("resp: {:#?}", resp.json::<serde_json::Value>().await);
                Err(Error::ResponseError(status))
            }
        }
    }

    /// GET /api/v1/messages/{messageId} endpoint
    pub fn get_message(&self) -> GetMessageBuilder<'_> {
        GetMessageBuilder::new(self)
    }

    /// GET /api/v1/outputs/{outputId} endpoint
    /// Find an output by its transaction_id and corresponding output_index.
    pub async fn get_output(&self, output: &UTXOInput) -> Result<OutputMetadata> {
        let mut url = self.get_node()?;
        url.set_path(&format!(
            "api/v1/outputs/{}{}",
            output.id().to_string(),
            hex::encode(output.index().to_le_bytes())
        ));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let raw = resp.json::<Response<RawOutput>>().await?.data;
                Ok(OutputMetadata {
                    message_id: hex::decode(raw.message_id)?,
                    transaction_id: hex::decode(raw.transaction_id)?,
                    output_index: raw.output_index,
                    is_spent: raw.is_spent,
                    amount: raw.output.amount,
                    address: {
                        if raw.output.type_ == 0 && raw.output.address.type_ == 1 {
                            let mut address = [0u8; ADDRESS_LENGTH];
                            hex::decode_to_slice(raw.output.address.address, &mut address)?;
                            Address::from(Ed25519Address::from(address))
                        } else {
                            return Err(Error::InvalidParameter("address type".to_string()));
                        }
                    },
                })
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// GET /api/v1/addresses/{address} endpoint
    pub fn get_address<'a>(&'a self) -> GetAddressBuilder<'a> {
        GetAddressBuilder::new(self)
    }

    /// GET /api/v1/milestones/{index} endpoint
    /// Get the milestone by the given index.
    pub async fn get_milestone(&self, index: u64) -> Result<MilestoneMetadata> {
        let mut url = self.get_node()?;
        url.set_path(&format!("api/v1/milestones/{}", index));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let milestone = resp.json::<Response<MilestoneMetadata>>().await?.data;
                Ok(milestone)
            }
            status => Err(Error::ResponseError(status)),
        }
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

    // /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    // /// confirmed for a while.
    // pub async fn reattach(&self, message_id: &MessageId) -> Result<Message> {
    //     let message = self.get_message().data(message_id).await?;
    //     self.post_messages(&message).await?;
    //     Ok(message)
    // }

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

// pub(crate) fn hex_to_message_id(data: &str) -> Result<MessageId> {
//     let mut m = [0; MESSAGE_ID_LENGTH];
//     hex::decode_to_slice(data, &mut m)?;
//     Ok(MessageId::new(m))
// }
