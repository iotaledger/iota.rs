// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Client module to connect through HORNET or Bee with API usages
use crate::{
    api::*,
    builder::{ClientBuilder, NetworkInfo, GET_API_TIMEOUT},
    error::*,
    node::*,
};
use bee_common::packable::Packable;
use bee_message::prelude::{
    Address, Ed25519Address, Message, MessageBuilder, MessageId, Parents, TransactionId, UtxoInput,
};
use bee_pow::providers::{MinerBuilder, Provider as PowProvider, ProviderBuilder as PowProviderBuilder};
use bee_rest_api::types::{
    dtos::{MessageDto, PeerDto, ReceiptDto},
    responses::{
        BalanceAddressResponse, InfoResponse as NodeInfo, MilestoneResponse as MilestoneResponseDto, OutputResponse,
        ReceiptsResponse, TipsResponse, TreasuryResponse, UtxoChangesResponse as MilestoneUTXOChanges,
    },
};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::Seed,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[cfg(feature = "mqtt")]
use rumqttc::AsyncClient as MqttClient;
use tokio::{
    runtime::Runtime,
    sync::{
        broadcast::{Receiver, Sender},
        watch::{Receiver as WatchReceiver, Sender as WatchSender},
        RwLock,
    },
    time::{sleep, Duration as TokioDuration},
};
#[cfg(all(feature = "sync", not(feature = "async")))]
use ureq::{Agent, AgentBuilder};
use url::Url;

use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    hash::Hash,
    str::FromStr,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

#[derive(Debug, Serialize)]
/// Milestone data.
pub struct MilestoneResponse {
    /// Milestone index.
    pub index: u32,
    /// Milestone message id.
    pub message_id: MessageId,
    /// Milestone timestamp.
    pub timestamp: u64,
}

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

/// Mqtt events.
#[cfg(feature = "mqtt")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MqttEvent {
    /// Client was connected.
    Connected,
    /// Client was disconnected.
    Disconnected,
}

/// The MQTT broker options.
#[cfg(feature = "mqtt")]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BrokerOptions {
    #[serde(default = "default_broker_automatic_disconnect", rename = "automaticDisconnect")]
    pub(crate) automatic_disconnect: bool,
    #[serde(default = "default_broker_timeout")]
    pub(crate) timeout: Duration,
    #[serde(rename = "maxReconnectionAttempts", default)]
    pub(crate) max_reconnection_attempts: usize,
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
fn default_max_reconnection_attempts() -> usize {
    3
}

#[cfg(feature = "mqtt")]
impl Default for BrokerOptions {
    fn default() -> Self {
        Self {
            automatic_disconnect: default_broker_automatic_disconnect(),
            timeout: default_broker_timeout(),
            max_reconnection_attempts: default_max_reconnection_attempts(),
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

    /// Sets the maximum number of reconnection attempts.
    pub fn max_reconnection_attempts(mut self, max_reconnection_attempts: usize) -> Self {
        self.max_reconnection_attempts = max_reconnection_attempts;
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

    fn nonce(
        &self,
        bytes: &[u8],
        target_score: f64,
        done: Option<Arc<AtomicBool>>,
    ) -> std::result::Result<u64, Self::Error> {
        if self.local_pow {
            MinerBuilder::new()
                .with_num_workers(num_cpus::get())
                .finish()
                .nonce(bytes, target_score, done)
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
    /// `get_peers`API
    GetPeers,
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
    /// `get_message` API
    GetMessage,
    /// `get_balance` API
    GetBalance,
}

impl FromStr for Api {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let t = match s {
            "GetHealth" => Self::GetHealth,
            "GetInfo" => Self::GetInfo,
            "GetPeers" => Self::GetPeers,
            "GetTips" => Self::GetTips,
            "PostMessage" => Self::PostMessage,
            "PostMessageWithRemotePow" => Self::PostMessageWithRemotePow,
            "GetOutput" => Self::GetOutput,
            "GetMilestone" => Self::GetMilestone,
            "GetMessage" => Self::GetMessage,
            "GetBalance" => Self::GetBalance,
            _ => return Err(format!("unknown api kind `{}`", s)),
        };
        Ok(t)
    }
}

#[cfg(all(feature = "sync", not(feature = "async")))]
pub(crate) struct Response(ureq::Response);

#[cfg(all(feature = "sync", not(feature = "async")))]
impl From<ureq::Response> for Response {
    fn from(response: ureq::Response) -> Self {
        Self(response)
    }
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl Response {
    pub(crate) fn status(&self) -> u16 {
        self.0.status()
    }

    pub(crate) async fn json<T: DeserializeOwned>(self) -> Result<T> {
        self.0.into_json().map_err(Into::into)
    }

    pub(crate) async fn text(self) -> Result<String> {
        self.0.into_string().map_err(Into::into)
    }
}

#[cfg(feature = "async")]
pub(crate) struct Response(reqwest::Response);

#[cfg(feature = "async")]
impl Response {
    pub(crate) fn status(&self) -> u16 {
        self.0.status().as_u16()
    }

    pub(crate) async fn json<T: DeserializeOwned>(self) -> Result<T> {
        self.0.json().await.map_err(Into::into)
    }

    pub(crate) async fn text(self) -> Result<String> {
        self.0.text().await.map_err(Into::into)
    }
}

#[cfg(feature = "async")]
pub(crate) struct HttpClient {
    client: reqwest::Client,
}

#[cfg(all(feature = "sync", not(feature = "async")))]
pub(crate) struct HttpClient;

#[cfg(feature = "async")]
impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn parse_response(response: reqwest::Response) -> Result<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(Response(response))
        } else {
            Err(Error::ResponseError(status.as_u16(), response.text().await?))
        }
    }

    pub(crate) async fn get(&self, url: &str, timeout: Duration) -> Result<Response> {
        Self::parse_response(self.client.get(url).timeout(timeout).send().await?).await
    }

    pub(crate) async fn post_bytes(&self, url: &str, timeout: Duration, body: &[u8]) -> Result<Response> {
        Self::parse_response(
            self.client
                .post(url)
                .timeout(timeout)
                .body(body.to_vec())
                .send()
                .await?,
        )
        .await
    }

    pub(crate) async fn post_json(&self, url: &str, timeout: Duration, json: Value) -> Result<Response> {
        Self::parse_response(self.client.post(url).timeout(timeout).json(&json).send().await?).await
    }
}

#[cfg(all(feature = "sync", not(feature = "async")))]
impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn get(&self, url: &str, timeout: Duration) -> Result<Response> {
        Ok(Self::get_ureq_agent(timeout).get(url).call()?.into())
    }

    pub(crate) async fn post_bytes(&self, url: &str, timeout: Duration, body: &[u8]) -> Result<Response> {
        Ok(Self::get_ureq_agent(timeout).post(url).send_bytes(body)?.into())
    }

    pub(crate) async fn post_json(&self, url: &str, timeout: Duration, json: Value) -> Result<Response> {
        Ok(Self::get_ureq_agent(timeout).post(url).send_json(json)?.into())
    }

    fn get_ureq_agent(timeout: Duration) -> Agent {
        AgentBuilder::new().timeout_read(timeout).timeout_write(timeout).build()
    }
}

/// An instance of the client using HORNET or Bee URI
pub struct Client {
    #[allow(dead_code)]
    pub(crate) runtime: Option<Runtime>,
    /// Node pool.
    pub(crate) nodes: HashSet<Url>,
    /// Node pool of synced IOTA nodes
    pub(crate) sync: Arc<RwLock<HashSet<Url>>>,
    /// Flag to stop the node syncing
    pub(crate) sync_kill_sender: Option<Arc<Sender<()>>>,
    /// A MQTT client to subscribe/unsubscribe to topics.
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_client: Option<MqttClient>,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_topic_handlers: Arc<RwLock<TopicHandlerMap>>,
    #[cfg(feature = "mqtt")]
    pub(crate) broker_options: BrokerOptions,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_event_channel: (Arc<WatchSender<MqttEvent>>, WatchReceiver<MqttEvent>),
    pub(crate) network_info: Arc<RwLock<NetworkInfo>>,
    /// HTTP request timeout.
    pub(crate) request_timeout: Duration,
    /// HTTP request timeout for each API call.
    pub(crate) api_timeout: HashMap<Api, Duration>,
    /// HTTP client.
    pub(crate) http_client: HttpClient,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Client");
        d.field("sync", &self.sync);
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
        if let Some(mqtt_client) = self.mqtt_client.take() {
            std::thread::spawn(move || {
                // ignore errors in case the event loop was already dropped
                // .cancel() finishes the event loop right away
                let _ = crate::async_runtime::block_on(mqtt_client.cancel());
            })
            .join()
            .unwrap();
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
        let node_sync_interval = TokioDuration::from_nanos(
            node_sync_interval
                .as_nanos()
                .try_into()
                .expect("Node sync interval parsing failed."),
        );

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
        let mut network_nodes: HashMap<String, Vec<(NodeInfo, Url)>> = HashMap::new();
        for node_url in nodes {
            // Put the healthy node url into the network_nodes
            if let Ok(info) = Client::get_node_info(&node_url.to_string()).await {
                if info.is_healthy {
                    match network_nodes.get_mut(&info.network_id) {
                        Some(network_id_entry) => {
                            network_id_entry.push((info, node_url.clone()));
                        }
                        None => match &network_info.read().await.network {
                            Some(id) => {
                                if info.network_id.contains(id) {
                                    network_nodes.insert(info.network_id.clone(), vec![(info, node_url.clone())]);
                                }
                            }
                            None => {
                                network_nodes.insert(info.network_id.clone(), vec![(info, node_url.clone())]);
                            }
                        },
                    }
                }
            }
        }
        // Get network_id with the most nodes
        let mut most_nodes = ("network_id", 0);
        for (network_id, node) in network_nodes.iter() {
            if node.len() > most_nodes.1 {
                most_nodes.0 = network_id;
                most_nodes.1 = node.len();
            }
        }
        if let Some(nodes) = network_nodes.get(most_nodes.0) {
            for (info, node_url) in nodes.iter() {
                let mut client_network_info = network_info.write().await;
                client_network_info.network_id = hash_network(&info.network_id).ok();
                client_network_info.min_pow_score = info.min_pow_score;
                client_network_info.bech32_hrp = info.bech32_hrp.clone();
                if !client_network_info.local_pow {
                    if info.features.contains(&"PoW".to_string()) {
                        synced_nodes.insert(node_url.clone());
                    }
                } else {
                    synced_nodes.insert(node_url.clone());
                }
            }
        }

        // Update the sync list
        *sync.write().await = synced_nodes;
    }

    /// Get a node candidate from the synced node pool.
    pub(crate) async fn get_node(&self) -> Result<Url> {
        let pool = self.sync.read().await;
        Ok(pool.iter().next().ok_or(Error::SyncedNodePoolEmpty)?.clone())
    }

    /// Gets the network id of the node we're connecting to.
    pub async fn get_network_id(&self) -> Result<u64> {
        let network_info = self.get_network_info().await?;
        network_info
            .network_id
            .ok_or(Error::MissingParameter("Missing network id."))
    }

    /// Gets the miner to use based on the PoW setting
    pub async fn get_pow_provider(&self) -> ClientMiner {
        ClientMinerBuilder::new()
            .with_local_pow(self.get_local_pow().await)
            .finish()
    }

    /// Gets the network related information such as network_id and min_pow_score
    /// and if it's the default one, sync it first.
    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        let not_synced = { self.network_info.read().await.network_id.is_none() };
        if not_synced {
            let info = self.get_info().await?;
            let network_id = hash_network(&info.network_id).ok();
            let mut client_network_info = self.network_info.write().await;
            client_network_info.network_id = network_id;
            client_network_info.min_pow_score = info.min_pow_score;
            client_network_info.bech32_hrp = info.bech32_hrp;
        }
        Ok(self.network_info.read().await.clone())
    }

    /// returns the bech32_hrp
    pub async fn get_bech32_hrp(&self) -> Result<String> {
        Ok(self.get_network_info().await?.bech32_hrp)
    }

    /// returns the min pow score
    pub async fn get_min_pow_score(&self) -> Result<f64> {
        Ok(self.get_network_info().await?.min_pow_score)
    }

    /// returns the tips interval
    pub async fn get_tips_interval(&self) -> u64 {
        self.network_info.read().await.tips_interval
    }

    /// returns the local pow
    pub async fn get_local_pow(&self) -> bool {
        self.network_info.read().await.local_pow
    }

    /// returns the unsynced nodes.
    pub async fn unsynced_nodes(&self) -> HashSet<&Url> {
        let synced = self.sync.read().await;
        self.nodes.iter().filter(|node| !synced.contains(node)).collect()
    }

    ///////////////////////////////////////////////////////////////////////
    // MQTT API
    //////////////////////////////////////////////////////////////////////

    /// Returns a handle to the MQTT topics manager.
    #[cfg(feature = "mqtt")]
    pub fn subscriber(&mut self) -> MqttManager<'_> {
        MqttManager::new(self)
    }

    /// Returns the mqtt event receiver.
    #[cfg(feature = "mqtt")]
    pub fn mqtt_event_receiver(&self) -> WatchReceiver<MqttEvent> {
        self.mqtt_event_channel.1.clone()
    }

    //////////////////////////////////////////////////////////////////////
    // Node API
    //////////////////////////////////////////////////////////////////////

    pub(crate) fn get_timeout(&self, api: Api) -> Duration {
        *self.api_timeout.get(&api).unwrap_or(&self.request_timeout)
    }

    /// GET /health endpoint
    pub async fn get_node_health(url: &str) -> Result<bool> {
        let mut url = Url::parse(url)?;
        url.set_path("health");
        let status = HttpClient::new().get(url.as_str(), GET_API_TIMEOUT).await?.status();
        match status {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /health endpoint
    pub async fn get_health(&self) -> Result<bool> {
        let mut url = self.get_node().await?;
        url.set_path("health");
        let status = self.http_client.get(url.as_str(), GET_API_TIMEOUT).await?.status();
        match status {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /api/v1/info endpoint
    pub async fn get_node_info(url: &str) -> Result<NodeInfo> {
        let mut url = Url::parse(url)?;
        let path = "api/v1/info";
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: NodeInfo,
        }
        let resp: ResponseWrapper = HttpClient::new()
            .get(url.as_str(), GET_API_TIMEOUT)
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/info endpoint
    pub async fn get_info(&self) -> Result<NodeInfo> {
        let mut url = self.get_node().await?;
        let path = "api/v1/info";
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: NodeInfo,
        }

        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), self.get_timeout(Api::GetInfo))
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/peers endpoint
    pub async fn get_peers(&self) -> Result<Vec<PeerDto>> {
        let mut url = self.get_node().await?;
        let path = "api/v1/peers";
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: Vec<PeerDto>,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), self.get_timeout(Api::GetPeers))
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/tips endpoint
    pub async fn get_tips(&self) -> Result<Vec<MessageId>> {
        let mut url = self.get_node().await?;
        let path = "api/v1/tips";
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: TipsResponse,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), self.get_timeout(Api::GetTips))
            .await?
            .json()
            .await?;

        let mut tips = Vec::new();
        for tip in resp.data.tip_message_ids {
            let mut new_tip = [0u8; 32];
            hex::decode_to_slice(tip, &mut new_tip)?;
            tips.push(MessageId::from(new_tip));
        }
        Ok(tips)
    }

    /// POST /api/v1/messages endpoint
    pub async fn post_message(&self, message: &Message) -> Result<MessageId> {
        let mut url = self.get_node().await?;
        let path = "api/v1/messages";
        url.set_path(path);

        let timeout = if self.get_local_pow().await {
            self.get_timeout(Api::PostMessage)
        } else {
            self.get_timeout(Api::PostMessageWithRemotePow)
        };
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageIdWrapper,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct MessageIdWrapper {
            #[serde(rename = "messageId")]
            message_id: String,
        }
        let resp: ResponseWrapper = self
            .http_client
            .post_bytes(url.as_str(), timeout, &message.pack_new())
            .await?
            .json()
            .await?;

        let mut message_id_bytes = [0u8; 32];
        hex::decode_to_slice(resp.data.message_id, &mut message_id_bytes)?;
        Ok(MessageId::from(message_id_bytes))
    }

    /// POST JSON to /api/v1/messages endpoint
    pub async fn post_message_json(&self, message: &Message) -> Result<MessageId> {
        let mut url = self.get_node().await?;
        let path = "api/v1/messages";
        url.set_path(path);

        let timeout = if self.get_local_pow().await {
            self.get_timeout(Api::PostMessage)
        } else {
            self.get_timeout(Api::PostMessageWithRemotePow)
        };
        let message = MessageDto::try_from(message).map_err(crate::Error::DtoError)?;
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageIdWrapper,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct MessageIdWrapper {
            #[serde(rename = "messageId")]
            message_id: String,
        }
        let resp: ResponseWrapper = self
            .http_client
            .post_json(url.as_str(), timeout, serde_json::to_value(message)?)
            .await?
            .json()
            .await?;

        let mut message_id_bytes = [0u8; 32];
        hex::decode_to_slice(resp.data.message_id, &mut message_id_bytes)?;
        Ok(MessageId::from(message_id_bytes))
    }

    /// GET /api/v1/messages/{messageId} endpoint
    pub fn get_message(&self) -> GetMessageBuilder<'_> {
        GetMessageBuilder::new(self)
    }

    /// GET /api/v1/outputs/{outputId} endpoint
    /// Find an output by its transaction_id and corresponding output_index.
    pub async fn get_output(&self, output_id: &UtxoInput) -> Result<OutputResponse> {
        let mut url = self.get_node().await?;
        let path = &format!(
            "api/v1/outputs/{}{}",
            output_id.output_id().transaction_id().to_string(),
            hex::encode(output_id.output_id().index().to_le_bytes())
        );
        url.set_path(path);

        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: OutputResponse,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), self.get_timeout(Api::GetOutput))
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceeds individual node limit.
    pub async fn find_outputs(&self, outputs: &[UtxoInput], addresses: &[String]) -> Result<Vec<OutputResponse>> {
        let mut output_metadata = Vec::<OutputResponse>::new();
        // Use a `HashSet` to prevent duplicate output.
        let mut output_to_query = HashSet::<UtxoInput>::new();

        // Collect the `UtxoInput` in the HashSet.
        for output in outputs {
            output_to_query.insert(output.to_owned());
        }

        // Use `get_address()` API to get the address outputs first,
        // then collect the `UtxoInput` in the HashSet.
        for address in addresses {
            let address_outputs = self.get_address().outputs(&address, Default::default()).await?;
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
    pub async fn get_milestone(&self, index: u32) -> Result<MilestoneResponse> {
        let mut url = self.get_node().await?;
        let path = &format!("api/v1/milestones/{}", index);
        url.set_path(path);

        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MilestoneResponseDto,
        }

        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), self.get_timeout(Api::GetMilestone))
            .await?
            .json()
            .await?;

        let milestone = resp.data;
        let mut message_id = [0u8; 32];
        hex::decode_to_slice(milestone.message_id, &mut message_id)?;
        Ok(MilestoneResponse {
            index: milestone.milestone_index,
            message_id: MessageId::new(message_id),
            timestamp: milestone.timestamp,
        })
    }

    /// GET /api/v1/milestones/{index}/utxo-changes endpoint
    /// Get the milestone by the given index.
    pub async fn get_milestone_utxo_changes(&self, index: u32) -> Result<MilestoneUTXOChanges> {
        let mut url = self.get_node().await?;
        let path = &format!("api/v1/milestones/{}/utxo-changes", index);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MilestoneUTXOChanges,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), self.get_timeout(Api::GetMilestone))
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/receipts endpoint
    /// Get all receipts.
    pub async fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let mut url = self.get_node().await?;
        let path = &"api/v1/receipts";
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: ReceiptsResponseWrapper,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct ReceiptsResponseWrapper {
            receipts: ReceiptsResponse,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), GET_API_TIMEOUT)
            .await?
            .json()
            .await?;

        Ok(resp.data.receipts.0)
    }

    /// GET /api/v1/receipts/{migratedAt} endpoint
    /// Get the receipts by the given milestone index.
    pub async fn get_receipts_migrated_at(&self, milestone_index: u32) -> Result<Vec<ReceiptDto>> {
        let mut url = self.get_node().await?;
        let path = &format!("api/v1/receipts/{}", milestone_index);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: ReceiptsResponseWrapper,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct ReceiptsResponseWrapper {
            receipts: ReceiptsResponse,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), GET_API_TIMEOUT)
            .await?
            .json()
            .await?;

        Ok(resp.data.receipts.0)
    }

    /// GET /api/v1/treasury endpoint
    /// Get the treasury output.
    pub async fn get_treasury(&self) -> Result<TreasuryResponse> {
        let mut url = self.get_node().await?;
        let path = "api/v1/treasury";
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: TreasuryResponse,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), GET_API_TIMEOUT)
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/transactions/{transactionId}/included-message
    /// Returns the included message of the transaction.
    pub async fn get_included_message(&self, transaction_id: &TransactionId) -> Result<Message> {
        let mut url = self.get_node().await?;
        let path = &format!("api/v1/transactions/{}/included-message", transaction_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: Message,
        }
        let resp: ResponseWrapper = self
            .http_client
            .get(url.as_str(), GET_API_TIMEOUT)
            .await?
            .json()
            .await?;

        Ok(resp.data)
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

    /// Reattach a message without checking if it should be reattached
    pub async fn reattach_unchecked(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Get the Message object by the MessageID.
        let message = self.get_message().data(message_id).await?;

        let reattach_message = finish_pow(self, message.payload().to_owned()).await?;

        // Post the modified
        let message_id = self.post_message(&reattach_message).await?;
        // Get message if we use remote PoW, because the node will change parents and nonce
        let msg = match self.get_local_pow().await {
            true => reattach_message,
            false => self.get_message().data(&message_id).await?,
        };
        Ok((message_id, msg))
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

    /// Promote a message without checking if it should be promoted
    pub async fn promote_unchecked(&self, message_id: &MessageId) -> Result<(MessageId, Message)> {
        // Create a new message (zero value message) for which one tip would be the actual message
        let mut tips = self.get_tips().await?;
        let min_pow_score = self.get_min_pow_score().await?;
        let network_id = self.get_network_id().await?;
        let nonce_provider = self.get_pow_provider().await;
        tips.push(*message_id);
        // Sort tips/parents
        tips.dedup();
        tips.sort_unstable_by_key(|a| a.pack_new());
        let promote_message = MessageBuilder::<ClientMiner>::new()
            .with_network_id(network_id)
            .with_parents(Parents::new(tips)?)
            .with_nonce_provider(nonce_provider, min_pow_score, None)
            .finish()
            .map_err(|_| Error::TransactionError)?;

        let message_id = self.post_message(&promote_message).await?;
        // Get message if we use remote PoW, because the node will change parents and nonce
        let msg = match self.get_local_pow().await {
            true => promote_message,
            false => self.get_message().data(&message_id).await?,
        };
        Ok((message_id, msg))
    }

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////

    /// A generic send function for easily sending transaction or indexation messages.
    pub fn message(&self) -> ClientMessageBuilder<'_> {
        ClientMessageBuilder::new(self)
    }

    /// Return a valid unspent address.
    pub fn get_unspent_address<'a>(&'a self, seed: &'a Seed) -> GetUnspentAddressBuilder<'a> {
        GetUnspentAddressBuilder::new(self, seed)
    }

    /// Return a list of addresses from the seed regardless of their validity.
    pub fn get_addresses<'a>(&'a self, seed: &'a Seed) -> GetAddressesBuilder<'a> {
        GetAddressesBuilder::new(seed).with_client(&self)
    }

    /// Find all messages by provided message IDs and/or indexation_keys.
    pub async fn find_messages<I: AsRef<[u8]>>(
        &self,
        indexation_keys: &[I],
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
            let message_ids = self.get_message().index(index).await?;
            for message_id in message_ids.iter() {
                message_ids_to_query.insert(message_id.to_owned());
            }
        }

        // Use `get_message().data()` API to get the `Message`.
        for message_id in message_ids_to_query {
            let message = self.get_message().data(&message_id).await?;
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
    pub async fn get_address_balances(&self, addresses: &[String]) -> Result<Vec<BalanceAddressResponse>> {
        let mut address_balance_pairs = Vec::new();
        for address in addresses {
            let balance_response = self.get_address().balance(&address).await?;
            address_balance_pairs.push(balance_response);
        }
        Ok(address_balance_pairs)
    }

    /// Transforms bech32 to hex
    pub fn bech32_to_hex(bech32: &str) -> crate::Result<String> {
        let address = Address::try_from_bech32(bech32)?;
        if let Address::Ed25519(ed) = address {
            return Ok(ed.to_string());
        }

        Err(crate::Error::FailedToParseBech32ToHex)
    }

    /// Transforms hex to bech32
    pub async fn hex_to_bech32(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        let address: Ed25519Address = hex.parse::<Ed25519Address>()?;
        match bech32_hrp {
            Some(hrp) => Ok(Address::Ed25519(address).to_bech32(hrp)),
            None => Ok(Address::Ed25519(address).to_bech32(self.get_bech32_hrp().await?.as_str())),
        }
    }

    /// Returns a valid Address parsed from a String.
    pub fn parse_bech32_address(address: &str) -> crate::Result<Address> {
        Ok(Address::try_from_bech32(address)?)
    }

    /// Checks if a String address is valid.
    pub fn is_address_valid(address: &str) -> bool {
        Address::try_from_bech32(address).is_ok()
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

    /// Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
    /// milestone). Default interval is 5 seconds and max attempts is 10. Returns reattached messages
    pub async fn retry_until_included(
        &self,
        message_id: &MessageId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> Result<Vec<(MessageId, Message)>> {
        // Attachments of the Message to check inclusion state
        let mut message_ids = vec![*message_id];
        // Reattached Messages that get returned
        let mut messages_with_id = Vec::new();
        for _ in 0..max_attempts.unwrap_or(10) {
            sleep(Duration::from_secs(interval.unwrap_or(5))).await;
            // Check inclusion state for each attachment
            let message_ids_len = message_ids.len();
            for (index, msg_id) in message_ids.clone().iter().enumerate() {
                let message_metadata = self.get_message().metadata(&msg_id).await?;
                if message_metadata.ledger_inclusion_state.is_some() {
                    return Ok(messages_with_id);
                }
                // Only reattach or promote latest attachment of the message
                if index == message_ids_len {
                    if message_metadata.should_promote.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        self.promote_unchecked(&message_ids.last().unwrap()).await?;
                    } else if message_metadata.should_reattach.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        let reattached = self.reattach_unchecked(&message_ids.last().unwrap()).await?;
                        message_ids.push(reattached.0);
                        messages_with_id.push(reattached);
                    }
                }
            }
        }
        Err(Error::TangleInclusionError(message_id.to_string()))
    }
}

/// Hash the network id str from the nodeinfo to an u64 for the messageBuilder
pub fn hash_network(network_id_string: &str) -> Result<u64> {
    let bytes = Blake2b256::digest(network_id_string.as_bytes())[0..8]
        .try_into()
        .map_err(|_e| Error::Blake2b256Error("Hashing the network id failed."))?;

    Ok(u64::from_le_bytes(bytes))
}
