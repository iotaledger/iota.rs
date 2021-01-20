// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Client module to connect through HORNET or Bee with API usages
use crate::{
    api::*,
    builder::{ClientBuilder, Network, NetworkInfo},
    error::*,
    node::*,
    parse_response,
    types::*,
};

use bee_message::prelude::{Address, Bech32Address, Ed25519Address, Message, MessageId, UTXOInput};
use bee_pow::providers::{MinerBuilder, Provider as PowProvider, ProviderBuilder as PowProviderBuilder};
use bee_signing_ext::Seed;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
#[cfg(feature = "mqtt")]
use paho_mqtt::Client as MqttClient;
use reqwest::{IntoUrl, Url};
use tokio::{
    runtime::Runtime,
    sync::broadcast::{Receiver, Sender},
    time::{sleep, Duration as TokioDuration},
};

use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    hash::Hash,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};

const ADDRESS_LENGTH: usize = 32;

#[cfg(feature = "mqtt")]
type TopicHandler = Box<dyn Fn(&TopicEvent) + Send + Sync>;
#[cfg(feature = "mqtt")]
pub(crate) type TopicHandlerMap = HashMap<Topic, Vec<Arc<TopicHandler>>>;

/// An event from a MQTT topic.
#[cfg(feature = "mqtt")]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TopicEvent {
    /// the MQTT topic.
    pub topic: String,
    /// The MQTT event payload.
    pub payload: String,
}

/// The MQTT broker options.
#[cfg(feature = "mqtt")]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BrokerOptions {
    #[serde(default = "default_broker_automatic_disconnect", rename = "automaticDisconnect")]
    pub(crate) automatic_disconnect: bool,
    #[serde(default = "default_broker_timeout")]
    pub(crate) timeout: Duration,
    #[serde(default = "default_use_ws")]
    pub(crate) use_ws: bool,
}

#[cfg(feature = "mqtt")]
fn default_broker_automatic_disconnect() -> bool {
    true
}

#[cfg(feature = "mqtt")]
fn default_broker_timeout() -> Duration {
    Duration::from_secs(30)
}

#[cfg(feature = "mqtt")]
fn default_use_ws() -> bool {
    true
}

#[cfg(feature = "mqtt")]
impl Default for BrokerOptions {
    fn default() -> Self {
        Self {
            automatic_disconnect: default_broker_automatic_disconnect(),
            timeout: default_broker_timeout(),
            use_ws: default_use_ws(),
        }
    }
}

#[cfg(feature = "mqtt")]
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

    /// Decid if websockets or tcp will be used for the connection
    pub fn use_websockets(mut self, use_ws: bool) -> Self {
        self.use_ws = use_ws;
        self
    }
}

/// The miner builder.
#[derive(Default)]
pub struct ClientMinerBuilder {
    local_pow: bool,
}

impl ClientMinerBuilder {
    /// Sets the local PoW config
    pub fn with_local_pow(mut self, value: bool) -> Self {
        self.local_pow = value;
        self
    }
}

impl PowProviderBuilder for ClientMinerBuilder {
    type Provider = ClientMiner;

    fn new() -> Self {
        Self::default()
    }

    fn finish(self) -> ClientMiner {
        ClientMiner {
            local_pow: self.local_pow,
        }
    }
}

/// The miner used for PoW
pub struct ClientMiner {
    local_pow: bool,
}

impl PowProvider for ClientMiner {
    type Builder = ClientMinerBuilder;
    type Error = crate::Error;

    fn nonce(&self, bytes: &[u8], target_score: f64) -> std::result::Result<u64, Self::Error> {
        if self.local_pow {
            MinerBuilder::new()
                .with_num_workers(num_cpus::get())
                .finish()
                .nonce(bytes, target_score)
                .map_err(|e| crate::Error::Pow(e.to_string()))
        } else {
            Ok(0)
        }
    }
}

/// Each of the node APIs the client uses.
#[derive(Eq, PartialEq, Hash)]
pub enum Api {
    /// `get_health` API
    GetHealth,
    /// `get_info`API
    GetInfo,
    /// `get_tips` API
    GetTips,
    /// `post_message` API
    PostMessage,
    /// `post_message` API with remote pow
    PostMessageWithRemotePow,
    /// `get_output` API
    GetOutput,
    /// `get_milestone` API
    GetMilestone,
}

impl FromStr for Api {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let t = match s {
            "GetHealth" => Self::GetHealth,
            "GetInfo" => Self::GetInfo,
            "GetTips" => Self::GetTips,
            "PostMessage" => Self::PostMessage,
            "PostMessageWithRemotePow" => Self::PostMessageWithRemotePow,
            "GetOutput" => Self::GetOutput,
            "GetMilestone" => Self::GetMilestone,
            _ => return Err(format!("unknown api kind `{}`", s)),
        };
        Ok(t)
    }
}

/// An instance of the client using HORNET or Bee URI
pub struct Client {
    #[allow(dead_code)]
    pub(crate) runtime: Option<Runtime>,
    /// Node pool of synced IOTA nodes
    pub(crate) sync: Arc<RwLock<HashSet<Url>>>,
    /// Flag to stop the node syncing
    pub(crate) sync_kill_sender: Option<Arc<Sender<()>>>,
    /// A reqwest Client to make Requests with
    pub(crate) client: reqwest::Client,
    /// A MQTT client to subscribe/unsubscribe to topics.
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_client: Option<MqttClient>,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_topic_handlers: Arc<RwLock<TopicHandlerMap>>,
    #[cfg(feature = "mqtt")]
    pub(crate) broker_options: BrokerOptions,
    pub(crate) network_info: Arc<RwLock<NetworkInfo>>,
    /// HTTP request timeout.
    pub(crate) request_timeout: Duration,
    /// HTTP request timeout for each API call.
    pub(crate) api_timeout: HashMap<Api, Duration>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Client");
        d.field("sync", &self.sync).field("client", &self.client);
        #[cfg(feature = "mqtt")]
        d.field("broker_options", &self.broker_options);
        d.field("network_info", &self.network_info).finish()
    }
}

impl Drop for Client {
    /// Gracefully shutdown the `Client`
    fn drop(&mut self) {
        if let Some(sender) = self.sync_kill_sender.take() {
            sender.send(()).expect("failed to stop syncing process");
        }

        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }

        #[cfg(feature = "mqtt")]
        if self.mqtt_client.is_some() {
            self.subscriber()
                .disconnect()
                .expect("failed to disconnect MQTT client");
        }
    }
}

impl Client {
    /// Create the builder to instntiate the IOTA Client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Sync the node lists per node_sync_interval milliseconds
    pub(crate) fn start_sync_process(
        runtime: &Runtime,
        sync: Arc<RwLock<HashSet<Url>>>,
        nodes: HashSet<Url>,
        node_sync_interval: Duration,
        network_info: Arc<RwLock<NetworkInfo>>,
        mut kill: Receiver<()>,
    ) {
        let node_sync_interval = TokioDuration::from_nanos(node_sync_interval.as_nanos().try_into().unwrap());

        runtime.spawn(async move {
            loop {
                tokio::select! {
                    _ = async {
                            // delay first since the first `sync_nodes` call is made by the builder
                            // to ensure the node list is filled before the client is used
                            sleep(node_sync_interval).await;
                            Client::sync_nodes(&sync, &nodes, &network_info).await;
                    } => {}
                    _ = kill.recv() => {}
                }
            }
        });
    }

    pub(crate) async fn sync_nodes(
        sync: &Arc<RwLock<HashSet<Url>>>,
        nodes: &HashSet<Url>,
        network_info: &Arc<RwLock<NetworkInfo>>,
    ) {
        let mut synced_nodes = HashSet::new();

        for node_url in nodes {
            // Put the healty node url into the synced_nodes
            if let Ok(info) = Client::get_node_info(node_url.clone()).await {
                if info.is_healthy {
                    if network_info.read().unwrap().network == Network::Testnet && info.network_id == "mainnet"
                        || network_info.read().unwrap().network == Network::Mainnet && info.network_id != "mainnet"
                    {
                        continue;
                    }
                    let mut client_network_info = network_info.write().unwrap();
                    client_network_info.min_pow_score = info.min_pow_score;
                    client_network_info.bech32_hrp = info.bech32_hrp;
                    if !client_network_info.local_pow {
                        if info.features.contains(&"PoW".to_string()) {
                            synced_nodes.insert(node_url.clone());
                        }
                    } else {
                        synced_nodes.insert(node_url.clone());
                    }
                }
            }
        }

        // Update the sync list
        *sync.write().unwrap() = synced_nodes;
    }

    /// Get a node candidate from the synced node pool.
    pub(crate) fn get_node(&self) -> Result<Url> {
        let pool = self.sync.read().unwrap();
        Ok(pool.iter().next().ok_or(Error::SyncedNodePoolEmpty)?.clone())
    }

    /// Gets the network id of the node we're connecting to.
    pub async fn get_network_id(&self) -> Result<u64> {
        let info = self.get_info().await?;
        let mut hasher = VarBlake2b::new(32).unwrap();
        hasher.update(info.network_id.as_bytes());
        let mut result: [u8; 32] = [0; 32];
        hasher.finalize_variable(|res| {
            result = res.try_into().unwrap();
        });
        let network_id = u64::from_le_bytes(result[0..8].try_into().unwrap());
        Ok(network_id)
    }

    /// Gets the miner to use based on the PoW setting
    pub fn get_pow_provider(&self) -> ClientMiner {
        ClientMinerBuilder::new()
            .with_local_pow(self.network_info.read().unwrap().local_pow)
            .finish()
    }

    /// Gets the network related information such as network_id and min_pow_score
    pub fn get_network_info(&self) -> NetworkInfo {
        self.network_info.read().unwrap().clone()
    }

    ///////////////////////////////////////////////////////////////////////
    // MQTT API
    //////////////////////////////////////////////////////////////////////

    /// Returns a handle to the MQTT topics manager.
    #[cfg(feature = "mqtt")]
    pub fn subscriber(&mut self) -> MqttManager<'_> {
        MqttManager::new(self)
    }

    //////////////////////////////////////////////////////////////////////
    // Node API
    //////////////////////////////////////////////////////////////////////

    fn get_timeout(&self, api: Api) -> Duration {
        *self.api_timeout.get(&api).unwrap_or(&self.request_timeout)
    }

    /// GET /health endpoint
    pub async fn get_node_health<T: IntoUrl>(url: T) -> Result<bool> {
        let mut url = url.into_url()?;
        url.set_path("health");
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /health endpoint
    pub async fn get_health(&self) -> Result<bool> {
        let mut url = self.get_node()?;
        url.set_path("health");
        let resp = self
            .client
            .get(url)
            .timeout(self.get_timeout(Api::GetHealth))
            .send()
            .await?;

        match resp.status().as_u16() {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /api/v1/info endpoint
    pub async fn get_node_info<T: IntoUrl>(url: T) -> Result<NodeInfo> {
        let mut url = url.into_url()?;
        url.set_path("api/v1/info");
        let resp = reqwest::get(url).await?;

        parse_response!(resp, 200 => {
            Ok(resp.json::<Response<NodeInfo>>().await?.data)
        })
    }

    /// GET /api/v1/info endpoint
    pub async fn get_info(&self) -> Result<NodeInfo> {
        let mut url = self.get_node()?;
        url.set_path("api/v1/info");
        let resp = self
            .client
            .get(url)
            .timeout(self.get_timeout(Api::GetInfo))
            .send()
            .await?;

        parse_response!(resp, 200 => {
            Ok(resp.json::<Response<NodeInfo>>().await?.data)
        })
    }

    /// GET /api/v1/tips endpoint
    pub async fn get_tips(&self) -> Result<(MessageId, MessageId)> {
        let mut url = self.get_node()?;
        url.set_path("api/v1/tips");
        let resp = self
            .client
            .get(url)
            .timeout(self.get_timeout(Api::GetTips))
            .send()
            .await?;

        parse_response!(resp, 200 => {
            let pair = resp.json::<Response<Tips>>().await?.data;
            let (mut tip1, mut tip2) = ([0u8; 32], [0u8; 32]);
            hex::decode_to_slice(pair.tip1, &mut tip1)?;
            hex::decode_to_slice(pair.tip2, &mut tip2)?;

            Ok((MessageId::from(tip1), MessageId::from(tip2)))
        })
    }

    /// POST /api/v1/messages endpoint
    pub async fn post_message(&self, message: &Message) -> Result<MessageId> {
        let mut url = self.get_node()?;
        url.set_path("api/v1/messages");

        let message: MessageJson = message.into();
        let mut timeout = self.get_timeout(Api::PostMessage);
        if self.network_info.read().unwrap().local_pow {
            timeout = self.get_timeout(Api::PostMessageWithRemotePow);
        }
        let resp = self
            .client
            .post(url)
            .timeout(timeout)
            .header("content-type", "application/json; charset=UTF-8")
            .json(&message)
            .send()
            .await?;

        parse_response!(resp, 201 => {
            let m = resp.json::<Response<PostMessageId>>().await?.data;
            let mut message_id = [0u8; 32];
            hex::decode_to_slice(m.message_id, &mut message_id)?;
            Ok(MessageId::from(message_id))
        })
    }

    /// GET /api/v1/messages/{messageId} endpoint
    pub fn get_message(&self) -> GetMessageBuilder<'_> {
        GetMessageBuilder::new(self)
    }

    /// GET /api/v1/outputs/{outputId} endpoint
    /// Find an output by its transaction_id and corresponding output_index.
    pub async fn get_output(&self, output_id: &UTXOInput) -> Result<OutputMetadata> {
        let mut url = self.get_node()?;
        url.set_path(&format!(
            "api/v1/outputs/{}{}",
            output_id.output_id().transaction_id().to_string(),
            hex::encode(output_id.output_id().index().to_le_bytes())
        ));
        let resp = self
            .client
            .get(url)
            .timeout(self.get_timeout(Api::GetOutput))
            .send()
            .await?;

        parse_response!(resp, 200 => {
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
        })
    }
    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceed individual node limit.
    pub async fn find_outputs(
        &self,
        outputs: &[UTXOInput],
        addresses: &[Bech32Address],
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
    pub fn get_address(&self) -> GetAddressBuilder<'_> {
        GetAddressBuilder::new(self)
    }

    /// GET /api/v1/milestones/{index} endpoint
    /// Get the milestone by the given index.
    pub async fn get_milestone(&self, index: u64) -> Result<MilestoneMetadata> {
        let mut url = self.get_node()?;
        url.set_path(&format!("api/v1/milestones/{}", index));
        let resp = self
            .client
            .get(url)
            .timeout(self.get_timeout(Api::GetMilestone))
            .send()
            .await?;

        parse_response!(resp, 200 => {
            let milestone = resp.json::<Response<MilestoneMetadata>>().await?.data;
            Ok(milestone)
        })
    }

    /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub async fn reattach(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        let metadata = self.get_message().metadata(message_id).await?;
        if metadata.should_reattach.unwrap_or(false) {
            self.reattach_unchecked(message_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(message_id.to_string()))
        }
    }

    async fn reattach_unchecked(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Get the Message object by the MessageID.
        let message = self.get_message().data(message_id).await?;

        // Change the fields of parent1 and parent2.
        let tips = self.get_tips().await?;
        let reattach_message = Message::builder()
            .with_network_id(self.get_network_id().await?)
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
        let metadata = self.get_message().metadata(message_id).await?;
        if metadata.should_promote.unwrap_or(false) {
            self.promote_unchecked(message_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(message_id.to_string()))
        }
    }

    async fn promote_unchecked(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Create a new message (zero value message) for which one tip would be the actual message
        let tips = self.get_tips().await?;
        let promote_message = Message::builder()
            .with_network_id(self.get_network_id().await?)
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

    /// A generic send function for easily sending transaction or indexation messages.
    pub fn send(&self) -> SendBuilder<'_> {
        SendBuilder::new(self)
    }

    /// Return a valid unspent address.
    pub fn get_unspent_address<'a>(&'a self, seed: &'a Seed) -> GetUnspentAddressBuilder<'a> {
        GetUnspentAddressBuilder::new(self, seed)
    }

    /// Return a list of addresses from the seed regardless of their validity.
    pub fn find_addresses<'a>(&'a self, seed: &'a Seed) -> GetAddressesBuilder<'a> {
        GetAddressesBuilder::new(self, seed)
    }

    /// Find all messages by provided message IDs and/or indexation_keys.
    pub async fn find_messages(&self, indexation_keys: &[String], message_ids: &[MessageId]) -> Result<Vec<Message>> {
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

    /// Return the balance for a provided seed and its wallet chain account index.
    /// Addresses with balance must be consecutive, so this method will return once it encounters a zero
    /// balance address.
    pub fn get_balance<'a>(&'a self, seed: &'a Seed) -> GetBalanceBuilder<'a> {
        GetBalanceBuilder::new(self, seed)
    }

    /// Return the balance in iota for the given addresses; No seed or security level needed to do this
    /// since we are only checking and already know the addresses.
    pub async fn get_address_balances(&self, addresses: &[Bech32Address]) -> Result<Vec<AddressBalancePair>> {
        let mut address_balance_pairs = Vec::new();
        for address in addresses {
            let balance = self.get_address().balance(&address).await?;
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
            self.promote_unchecked(message_id).await
        } else if message_metadata.should_reattach.unwrap_or(false) {
            self.reattach_unchecked(message_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(message_id.to_string()))
        }
    }
}
