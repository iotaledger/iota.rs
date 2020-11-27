//! The Client module to connect through IRI with API usages
use crate::api::*;
use crate::builder::ClientBuilder;
use crate::error::*;
pub use crate::node::Topic;
use crate::node::*;
use crate::types::*;

use bee_message::prelude::{Address, Ed25519Address, Message, MessageId, UTXOInput};
use bee_signing_ext::Seed;

use paho_mqtt::Client as MqttClient;
use reqwest::{IntoUrl, Url};

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

const ADDRESS_LENGTH: usize = 32;

type TopicHandler = Box<dyn Fn(&TopicEvent) + Send + Sync>;
pub(crate) type TopicHandlerMap = HashMap<Topic, Vec<Arc<TopicHandler>>>;

/// An event from a MQTT topic.
#[derive(Debug, Clone)]
pub struct TopicEvent {
    /// the MQTT topic.
    pub topic: String,
    /// The MQTT event payload.
    pub payload: String,
}

/// The MQTT broker options.
pub struct BrokerOptions {
    pub(crate) automatic_disconnect: bool,
    pub(crate) timeout: Duration,
}

impl Default for BrokerOptions {
    fn default() -> Self {
        Self {
            automatic_disconnect: true,
            timeout: Duration::from_secs(30),
        }
    }
}

impl BrokerOptions {
    /// Creates the default broker options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not.
    pub fn automatic_disconnect(mut self, automatic_disconnect: bool) -> Self {
        self.automatic_disconnect = automatic_disconnect;
        self
    }

    /// Sets the timeout used for the MQTT operations.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// An instance of the client using IRI URI
pub struct Client {
    /// Node pool of IOTA nodes
    pub(crate) pool: Arc<RwLock<HashSet<Url>>>,
    pub(crate) sync: Arc<RwLock<Vec<Url>>>,
    /// A reqwest Client to make Requests with
    pub(crate) client: reqwest::Client,
    pub(crate) mwm: u8,
    pub(crate) quorum_size: u8,
    pub(crate) quorum_threshold: u8,
    /// A MQTT client to subscribe/unsubscribe to topics.
    pub(crate) mqtt_client: Option<MqttClient>,
    pub(crate) mqtt_topic_handlers: Arc<Mutex<TopicHandlerMap>>,
    pub(crate) broker_options: BrokerOptions,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("pool", &self.pool)
            .field("sync", &self.sync)
            .field("client", &self.client)
            .field("mwm", &self.mwm)
            .field("quorum_size", &self.quorum_size)
            .field("quorum_threshold", &self.quorum_threshold)
            .finish()
    }
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

    ///////////////////////////////////////////////////////////////////////
    // MQTT API
    //////////////////////////////////////////////////////////////////////

    /// Returns a handle to the MQTT topics manager.
    pub fn subscriber(&mut self) -> MqttManager<'_> {
        MqttManager::new(self)
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
            output.output_id().transaction_id().to_string(),
            hex::encode(output.output_id().index().to_le_bytes())
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
    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceed individual node limit.
    pub async fn find_outputs(
        &self,
        outputs: &[UTXOInput],
        addresses: &[Address],
    ) -> Result<Vec<OutputMetadata>> {
        let mut output_metadata = Vec::<OutputMetadata>::new();
        // Use a `HashSet` to prevent duplicate output.
        let mut output_to_query = HashSet::<UTXOInput>::new();

        // Collect the `UTXOInput` in the HashSet.
        for output in outputs {
            output_to_query.insert(output.to_owned());
        }

        // Use `get_address()` API to get the address outputs first,
        // then collect the `UTXOInput` in the HashSet.
        for address in addresses {
            let address_outputs = self.get_address().outputs(&address).await?;
            for output in address_outputs.iter() {
                output_to_query.insert(output.to_owned());
            }
        }

        // Use `get_output` API to get the `OutputMetadata`.
        for output in output_to_query {
            let meta_data = self.get_output(&output).await?;
            output_metadata.push(meta_data);
        }
        Ok(output_metadata)
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

    /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub async fn reattach(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Get the Message object by the MessageID.
        let message = self.get_message().data(message_id).await?;

        // Change the fields of parent1 and parent2.
        let tips = self.get_tips().await?;
        let reattach_message = Message::builder()
            // TODO: make the newtwork id configurable
            .with_network_id(0)
            .with_parent1(tips.0)
            .with_parent2(tips.1)
            .with_payload(message.payload().to_owned().unwrap())
            .finish()
            .map_err(|_| Error::TransactionError)?;

        // Post the modified
        let message_id = self.post_message(&reattach_message).await?;
        Ok((message_id, reattach_message))
    }

    /// Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
    /// method should error out and should not allow unnecessary promotions.
    pub async fn promote(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Create a new message (zero value message) for which one tip would be the actual message
        let tips = self.get_tips().await?;
        let promote_message = Message::builder()
            // TODO: make the newtwork id configurable
            .with_network_id(0)
            .with_parent1(tips.0)
            .with_parent2(*message_id)
            .finish()
            .map_err(|_| Error::TransactionError)?;

        let message_id = self.post_message(&promote_message).await?;
        Ok((message_id, promote_message))
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
    pub fn find_addresses<'a>(&'a self, seed: &'a Seed) -> GetAddressesBuilder<'a> {
        GetAddressesBuilder::new(self, seed)
    }

    /// Find all messages by provided message IDs. This method will try to query multiple nodes
    /// if the request amount exceed individual node limit.
    pub async fn find_messages(
        &self,
        indexation_keys: &[String],
        message_ids: &[MessageId],
    ) -> Result<Vec<Message>> {
        let mut messages = Vec::new();

        // Use a `HashSet` to prevent duplicate message_ids.
        let mut message_ids_to_query = HashSet::<MessageId>::new();

        // Collect the `MessageId` in the HashSet.
        for message_id in message_ids {
            message_ids_to_query.insert(message_id.to_owned());
        }

        // Use `get_message().index()` API to get the message ID first,
        // then collect the `MessageId` in the HashSet.
        for index in indexation_keys {
            let message_ids = self.get_message().index(&index).await?;
            for message_id in message_ids.iter() {
                message_ids_to_query.insert(message_id.to_owned());
            }
        }

        // Use `get_message().data()` API to get the `Message`.
        for message_id in message_ids_to_query {
            let message = self.get_message().data(&message_id).await.unwrap();
            messages.push(message);
        }

        Ok(messages)
    }

    /// Return the balance for a provided seed and its wallet chain BIP32 path. BIP32 derivation path
    /// of the address should be in form of `m/0'/0'/k'`. So the wallet chain is expected to be `m/0'/0'`.
    /// Addresses with balance must be consecutive, so this method will return once it encounters a zero
    /// balance address.
    pub fn get_balance<'a>(&'a self, seed: &'a Seed) -> GetBalanceBuilder<'a> {
        GetBalanceBuilder::new(self, seed)
    }

    /// Return the balance in iota for the given addresses; No seed or security level needed to do this
    /// since we are only checking and already know the addresses.
    pub async fn get_address_balances(
        &self,
        addresses: &[Address],
    ) -> Result<Vec<AddressBalancePair>> {
        let mut address_balance_pairs = Vec::new();
        for address in addresses {
            let balance = self.get_address().balance(address).await?;
            address_balance_pairs.push(AddressBalancePair {
                address: address.clone(),
                balance,
            });
        }
        Ok(address_balance_pairs)
    }

    /// Retries (promotes or reattaches) a message for provided message id. Message should only be
    /// retried only if they are valid and haven't been confirmed for a while.
    pub async fn retry(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Get the metadata to check if it needs to promote or reattach
        let message_metadata = self.get_message().metadata(message_id).await?;
        if message_metadata.should_promote.unwrap_or(false) {
            return self.promote(message_id).await;
        } else if message_metadata.should_reattach.unwrap_or(false) {
            return self.reattach(message_id).await;
        } else {
            return Err(Error::NoNeedPromoteOrReattach(message_id.to_string()));
        }
    }
}

// pub(crate) fn hex_to_message_id(data: &str) -> Result<MessageId> {
//     let mut m = [0; MESSAGE_ID_LENGTH];
//     hex::decode_to_slice(data, &mut m)?;
//     Ok(MessageId::new(m))
// }
