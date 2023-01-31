// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Client module to connect through HORNET or Bee with API usages
use crate::{
    api::*,
    builder::{ClientBuilder, NetworkInfo, GET_API_TIMEOUT},
    error::*,
    node::*,
    node_manager::Node,
};
use bee_common::packable::Packable;
use bee_message::{
    constants::INPUT_OUTPUT_COUNT_MAX,
    payload::Payload,
    prelude::{
        Address, Ed25519Address, Message, MessageBuilder, MessageId, Parents, TransactionId, UtxoInput,
        ED25519_ADDRESS_LENGTH,
    },
};
use bee_pow::providers::{
    miner::{MinerBuilder, MinerCancel},
    NonceProvider, NonceProviderBuilder,
};
use bee_rest_api::types::{
    body::SuccessBody,
    dtos::{LedgerInclusionStateDto, MessageDto, PeerDto, ReceiptDto},
    responses::{
        BalanceAddressResponse, InfoResponse as NodeInfo, MessageResponse, MilestoneResponse as MilestoneResponseDto,
        OutputResponse, PeersResponse, ReceiptsResponse, SubmitMessageResponse, TipsResponse, TreasuryResponse,
        UtxoChangesResponse as MilestoneUTXOChanges,
    },
};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::Seed,
    },
    utils,
};

use zeroize::Zeroize;

use crate::builder::TIPS_INTERVAL;
#[cfg(feature = "wasm")]
use gloo_timers::future::TimeoutFuture;
#[cfg(feature = "mqtt")]
use rumqttc::AsyncClient as MqttClient;
#[cfg(feature = "mqtt")]
use tokio::sync::watch::{Receiver as WatchReceiver, Sender as WatchSender};
#[cfg(not(feature = "wasm"))]
use tokio::{
    runtime::Runtime,
    sync::broadcast::{Receiver, Sender},
    time::{sleep, Duration as TokioDuration},
};
use url::Url;

use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    hash::Hash,
    ops::Range,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};

const RESPONSE_MAX_OUTPUTS: usize = 1000;
const DUST_THRESHOLD: u64 = 1_000_000;

/// NodeInfo wrapper which contains the nodeinfo and the url from the node (useful when multiple nodes are used)
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfoWrapper {
    /// The returned nodeinfo
    pub nodeinfo: NodeInfo,
    /// The url from the node which returned the nodeinfo
    pub url: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
/// Milestone data.
pub struct MilestoneResponse {
    /// Milestone index.
    pub index: u32,
    /// Milestone message id.
    #[serde(rename = "messageId")]
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
    #[serde(default = "default_broker_use_ws", rename = "useWs")]
    pub(crate) use_ws: bool,
    #[serde(default = "default_broker_port")]
    pub(crate) port: u16,
    #[serde(default = "default_max_reconnection_attempts", rename = "maxReconnectionAttempts")]
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
fn default_broker_use_ws() -> bool {
    true
}

#[cfg(feature = "mqtt")]
fn default_broker_port() -> u16 {
    1883
}

#[cfg(feature = "mqtt")]
fn default_max_reconnection_attempts() -> usize {
    0
}

#[cfg(feature = "mqtt")]
impl Default for BrokerOptions {
    fn default() -> Self {
        Self {
            automatic_disconnect: default_broker_automatic_disconnect(),
            timeout: default_broker_timeout(),
            use_ws: default_broker_use_ws(),
            port: default_broker_port(),
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

    /// Sets the use_ws used for the MQTT operations.
    pub fn use_ws(mut self, use_ws: bool) -> Self {
        self.use_ws = use_ws;
        self
    }

    /// Sets the port used for the MQTT operations.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the maximum number of reconnection attempts. 0 is unlimited.
    pub fn max_reconnection_attempts(mut self, max_reconnection_attempts: usize) -> Self {
        self.max_reconnection_attempts = max_reconnection_attempts;
        self
    }
}

/// The miner builder.
#[derive(Default)]
pub struct ClientMinerBuilder {
    local_pow: bool,
    cancel: MinerCancel,
}

impl ClientMinerBuilder {
    /// Sets the local PoW config
    pub fn with_local_pow(mut self, value: bool) -> Self {
        self.local_pow = value;
        self
    }
    /// Set cancel miner
    pub fn with_cancel(mut self, cancel: MinerCancel) -> Self {
        self.cancel = cancel;
        self
    }
}

impl NonceProviderBuilder for ClientMinerBuilder {
    type Provider = ClientMiner;

    fn new() -> Self {
        Self::default()
    }

    fn finish(self) -> ClientMiner {
        ClientMiner {
            local_pow: self.local_pow,
            cancel: self.cancel,
        }
    }
}

/// The miner used for PoW
pub struct ClientMiner {
    local_pow: bool,
    cancel: MinerCancel,
}

impl NonceProvider for ClientMiner {
    type Builder = ClientMinerBuilder;
    type Error = crate::Error;

    fn nonce(&self, bytes: &[u8], target_score: f64) -> std::result::Result<u64, Self::Error> {
        if self.local_pow {
            MinerBuilder::new()
                .with_num_workers(num_cpus::get())
                .with_cancel(self.cancel.clone())
                .finish()
                .nonce(bytes, target_score)
                .map_err(|e| crate::Error::Pow(e.to_string()))
        } else {
            Ok(0)
        }
    }
}

/// Each of the node APIs the client uses.
#[derive(Clone, Eq, PartialEq, Hash)]
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
            _ => return Err(format!("unknown api kind `{s}`")),
        };
        Ok(t)
    }
}

/// An instance of the client using HORNET or Bee URI
#[cfg_attr(feature = "wasm", derive(Clone))]
pub struct Client {
    #[allow(dead_code)]
    #[cfg(not(feature = "wasm"))]
    pub(crate) runtime: Option<Runtime>,
    /// Node manager
    pub(crate) node_manager: crate::node_manager::NodeManager,
    /// Flag to stop the node syncing
    #[cfg(not(feature = "wasm"))]
    pub(crate) sync_kill_sender: Option<Arc<Sender<()>>>,
    /// A MQTT client to subscribe/unsubscribe to topics.
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_client: Option<MqttClient>,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_topic_handlers: Arc<tokio::sync::RwLock<TopicHandlerMap>>,
    #[cfg(feature = "mqtt")]
    pub(crate) broker_options: BrokerOptions,
    #[cfg(feature = "mqtt")]
    pub(crate) mqtt_event_channel: (Arc<WatchSender<MqttEvent>>, WatchReceiver<MqttEvent>),
    pub(crate) network_info: Arc<RwLock<NetworkInfo>>,
    /// HTTP request timeout.
    pub(crate) request_timeout: Duration,
    /// HTTP request timeout for each API call.
    pub(crate) api_timeout: HashMap<Api, Duration>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Client");
        d.field("node_manager", &self.node_manager);
        #[cfg(feature = "mqtt")]
        d.field("broker_options", &self.broker_options);
        d.field("network_info", &self.network_info).finish()
    }
}

impl Drop for Client {
    /// Gracefully shutdown the `Client`
    fn drop(&mut self) {
        #[cfg(not(feature = "wasm"))]
        if let Some(sender) = self.sync_kill_sender.take() {
            sender.send(()).expect("failed to stop syncing process");
        }

        #[cfg(not(feature = "wasm"))]
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
    #[cfg(not(feature = "wasm"))]
    pub(crate) fn start_sync_process(
        runtime: &Runtime,
        sync: Arc<RwLock<HashSet<Node>>>,
        nodes: HashSet<Node>,
        node_sync_interval: Duration,
        network_info: Arc<RwLock<NetworkInfo>>,
        mut kill: Receiver<()>,
    ) {
        let node_sync_interval =
            TokioDuration::from_nanos(node_sync_interval.as_nanos().try_into().unwrap_or(TIPS_INTERVAL));

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

    #[cfg(not(feature = "wasm"))]
    pub(crate) async fn sync_nodes(
        sync: &Arc<RwLock<HashSet<Node>>>,
        nodes: &HashSet<Node>,
        network_info: &Arc<RwLock<NetworkInfo>>,
    ) {
        let mut synced_nodes = HashSet::new();
        let mut network_nodes: HashMap<String, Vec<(NodeInfo, Node)>> = HashMap::new();
        for node in nodes {
            // Put the healthy node url into the network_nodes
            if let Ok(info) = Client::get_node_info(node.url.as_ref(), None, None).await {
                if info.is_healthy {
                    match network_nodes.get_mut(&info.network_id) {
                        Some(network_id_entry) => {
                            network_id_entry.push((info, node.clone()));
                        }
                        None => match &network_info
                            .read()
                            .map_or(NetworkInfo::default().network, |info| info.network.clone())
                        {
                            Some(id) => {
                                if info.network_id.contains(id) {
                                    network_nodes.insert(info.network_id.clone(), vec![(info, node.clone())]);
                                }
                            }
                            None => {
                                network_nodes.insert(info.network_id.clone(), vec![(info, node.clone())]);
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
                if let Ok(mut client_network_info) = network_info.write() {
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
        }

        // Update the sync list
        if let Ok(mut sync) = sync.write() {
            *sync = synced_nodes;
        }
    }

    /// Get a node candidate from the synced node pool.
    pub async fn get_node(&self) -> Result<Node> {
        if let Some(primary_node) = &self.node_manager.primary_node {
            return Ok(primary_node.clone());
        }
        let pool = self.node_manager.nodes.clone();
        pool.into_iter().next().ok_or(Error::SyncedNodePoolEmpty)
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
        let not_synced = self.network_info.read().map_or(true, |info| info.network_id.is_none());

        if not_synced {
            let info = self.get_info().await?.nodeinfo;
            let network_id = hash_network(&info.network_id).ok();
            {
                let mut client_network_info = self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                client_network_info.network_id = network_id;
                client_network_info.min_pow_score = info.min_pow_score;
                client_network_info.bech32_hrp = info.bech32_hrp;
            }
        }
        let res = self
            .network_info
            .read()
            .map_or(NetworkInfo::default(), |info| info.clone());
        Ok(res)
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
        self.network_info
            .read()
            .map_or(TIPS_INTERVAL, |info| info.tips_interval)
    }

    /// returns the local pow
    pub async fn get_local_pow(&self) -> bool {
        self.network_info
            .read()
            .map_or(NetworkInfo::default().local_pow, |info| info.local_pow)
    }

    /// returns the fallback_to_local_pow
    pub async fn get_fallback_to_local_pow(&self) -> bool {
        self.network_info
            .read()
            .map_or(NetworkInfo::default().fallback_to_local_pow, |info| {
                info.fallback_to_local_pow
            })
    }

    /// returns the unsynced nodes.
    #[cfg(not(feature = "wasm"))]
    pub async fn unsynced_nodes(&self) -> HashSet<&Node> {
        self.node_manager.synced_nodes.read().map_or(HashSet::new(), |synced| {
            self.node_manager
                .nodes
                .iter()
                .filter(|node| !synced.contains(node))
                .collect()
        })
    }

    /// Generates a new mnemonic.
    pub fn generate_mnemonic() -> Result<String> {
        let mut entropy = [0u8; 32];
        utils::rand::fill(&mut entropy)?;
        let mnemonic = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
            .map_err(|e| crate::Error::MnemonicError(format!("{e:?}")))?;
        entropy.zeroize();
        Ok(mnemonic)
    }

    /// Returns a hex encoded seed for a mnemonic.
    pub fn mnemonic_to_hex_seed(mnemonic: &str) -> Result<String> {
        // trim because empty spaces could create a different seed https://github.com/iotaledger/crypto.rs/issues/125
        let mnemonic = mnemonic.trim();
        // first we check if the mnemonic is valid to give meaningful errors
        crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
            .map_err(|e| crate::Error::InvalidMnemonic(format!("{e:?}")))?;
        let mut mnemonic_seed = [0u8; 64];
        mnemonic_to_seed(mnemonic, "", &mut mnemonic_seed);
        Ok(hex::encode(mnemonic_seed))
    }

    /// Function to find inputs from addresses for a provided amount (useful for offline signing)
    pub async fn find_inputs(&self, addresses: Vec<String>, amount: u64) -> Result<Vec<UtxoInput>> {
        // Get outputs from node and select inputs
        let mut available_outputs = Vec::new();
        for address in addresses {
            available_outputs.extend_from_slice(&self.get_address().outputs(&address, Default::default()).await?);
        }

        let mut signature_locked_outputs = Vec::new();
        let mut dust_allowance_outputs = Vec::new();

        for output in available_outputs.into_iter() {
            let output_data = self.get_output(&output).await?;
            let (amount, _, signature_locked) =
                ClientMessageBuilder::get_output_amount_and_address(&output_data.output)?;
            if signature_locked {
                signature_locked_outputs.push((output, amount));
            } else {
                dust_allowance_outputs.push((output, amount));
            }
        }
        signature_locked_outputs.sort_by(|l, r| r.1.cmp(&l.1));
        dust_allowance_outputs.sort_by(|l, r| r.1.cmp(&l.1));

        let mut total_already_spent = 0;
        let mut selected_inputs = Vec::new();
        for (_offset, output_wrapper) in signature_locked_outputs
            .into_iter()
            .chain(dust_allowance_outputs.into_iter())
            // Max inputs is 127
            .take(INPUT_OUTPUT_COUNT_MAX)
            .enumerate()
        {
            // Break if we have enough funds and don't create dust for the remainder
            if total_already_spent == amount || total_already_spent >= amount + DUST_THRESHOLD {
                break;
            }
            selected_inputs.push(output_wrapper.0.clone());
            total_already_spent += output_wrapper.1;
        }

        if total_already_spent < amount
            || (total_already_spent != amount && total_already_spent < amount + DUST_THRESHOLD)
        {
            return Err(crate::Error::NotEnoughBalance(total_already_spent, amount));
        }

        Ok(selected_inputs)
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
        let status = crate::node_manager::HttpClient::new()
            .get(Node { url, jwt: None }, GET_API_TIMEOUT)
            .await?
            .status();
        match status {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /health endpoint
    pub async fn get_health(&self) -> Result<bool> {
        let mut node = self.get_node().await?;
        node.url.set_path("health");
        let status = self.node_manager.http_client.get(node, GET_API_TIMEOUT).await?.status();
        match status {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// GET /api/v1/info endpoint
    pub async fn get_node_info(
        url: &str,
        jwt: Option<String>,
        auth_name_pwd: Option<(&str, &str)>,
    ) -> Result<NodeInfo> {
        let mut url = crate::node_manager::validate_url(Url::parse(url)?)?;
        if let Some((name, password)) = auth_name_pwd {
            url.set_username(name)
                .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
            url.set_password(Some(password))
                .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
        }

        let path = "api/v1/info";
        url.set_path(path);

        let resp: SuccessBody<NodeInfo> = crate::node_manager::HttpClient::new()
            .get(Node { url, jwt }, GET_API_TIMEOUT)
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/info endpoint
    pub async fn get_info(&self) -> Result<NodeInfoWrapper> {
        let path = "api/v1/info";

        let resp: NodeInfoWrapper = self
            .node_manager
            .get_request(path, None, self.get_timeout(Api::GetInfo))
            .await?;

        Ok(resp)
    }

    /// GET /api/v1/peers endpoint
    pub async fn get_peers(&self) -> Result<Vec<PeerDto>> {
        let path = "api/v1/peers";

        let resp: SuccessBody<PeersResponse> = self
            .node_manager
            .get_request(path, None, self.get_timeout(Api::GetPeers))
            .await?;

        Ok(resp.data.0)
    }

    /// GET /api/v1/tips endpoint
    pub async fn get_tips(&self) -> Result<Vec<MessageId>> {
        let path = "api/v1/tips";

        let resp: SuccessBody<TipsResponse> = self
            .node_manager
            .get_request(path, None, self.get_timeout(Api::GetTips))
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
        let path = "api/v1/messages";
        let local_pow = self.get_local_pow().await;
        let timeout = if local_pow {
            self.get_timeout(Api::PostMessage)
        } else {
            self.get_timeout(Api::PostMessageWithRemotePow)
        };

        // fallback to local PoW if remote PoW fails
        let resp: SuccessBody<SubmitMessageResponse> = match self
            .node_manager
            .post_request_bytes(path, timeout, &message.pack_new(), local_pow)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                if let Error::NodeError(e) = e {
                    let fallback_to_local_pow = self.get_fallback_to_local_pow().await;
                    // hornet and bee return different error messages
                    if (e == *"No available nodes with remote PoW"
                        || e.contains("proof of work is not enabled")
                        || e.contains("`PoW` not enabled"))
                        && fallback_to_local_pow
                    {
                        // Without this we get:within `impl Future<Output = [async output]>`, the trait `Send` is not
                        // implemented for `std::sync::RwLockWriteGuard<'_, NetworkInfo>`
                        {
                            let mut client_network_info =
                                self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                            // switch to local PoW
                            client_network_info.local_pow = true;
                        }
                        #[cfg(not(feature = "wasm"))]
                        let msg_res = crate::api::finish_pow(self, message.payload().clone()).await;
                        #[cfg(feature = "wasm")]
                        let msg_res = {
                            let min_pow_score = self.get_min_pow_score().await?;
                            let network_id = self.get_network_id().await?;
                            crate::api::finish_single_thread_pow(
                                self,
                                network_id,
                                None,
                                message.payload().clone(),
                                min_pow_score,
                            )
                            .await
                        };
                        let message_with_local_pow = match msg_res {
                            Ok(msg) => {
                                // reset local PoW state
                                let mut client_network_info =
                                    self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                                client_network_info.local_pow = false;
                                msg
                            }
                            Err(e) => {
                                // reset local PoW state
                                let mut client_network_info =
                                    self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                                client_network_info.local_pow = false;
                                return Err(e);
                            }
                        };
                        self.node_manager
                            .post_request_bytes(path, timeout, &message_with_local_pow.pack_new(), true)
                            .await?
                    } else {
                        return Err(Error::NodeError(e));
                    }
                } else {
                    return Err(e);
                }
            }
        };

        let mut message_id_bytes = [0u8; 32];
        hex::decode_to_slice(resp.data.message_id, &mut message_id_bytes)?;
        Ok(MessageId::from(message_id_bytes))
    }

    /// POST JSON to /api/v1/messages endpoint
    pub async fn post_message_json(&self, message: &Message) -> Result<MessageId> {
        let path = "api/v1/messages";
        let local_pow = self.get_local_pow().await;
        let timeout = if local_pow {
            self.get_timeout(Api::PostMessage)
        } else {
            self.get_timeout(Api::PostMessageWithRemotePow)
        };
        let message_dto = MessageDto::from(message);

        // fallback to local PoW if remote PoW fails
        let resp: SuccessBody<SubmitMessageResponse> = match self
            .node_manager
            .post_request_json(path, timeout, serde_json::to_value(message_dto)?, local_pow)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                if let Error::NodeError(e) = e {
                    let fallback_to_local_pow = self.get_fallback_to_local_pow().await;
                    // hornet and bee return different error messages
                    if (e == *"No available nodes with remote PoW"
                        || e.contains("proof of work is not enabled")
                        || e.contains("`PoW` not enabled"))
                        && fallback_to_local_pow
                    {
                        // Without this we get:within `impl Future<Output = [async output]>`, the trait `Send` is not
                        // implemented for `std::sync::RwLockWriteGuard<'_, NetworkInfo>`
                        {
                            let mut client_network_info =
                                self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                            // switch to local PoW
                            client_network_info.local_pow = true;
                        }
                        #[cfg(not(feature = "wasm"))]
                        let msg_res = crate::api::finish_pow(self, message.payload().clone()).await;
                        #[cfg(feature = "wasm")]
                        let msg_res = {
                            let min_pow_score = self.get_min_pow_score().await?;
                            let network_id = self.get_network_id().await?;
                            crate::api::finish_single_thread_pow(
                                self,
                                network_id,
                                None,
                                message.payload().clone(),
                                min_pow_score,
                            )
                            .await
                        };
                        let message_with_local_pow = match msg_res {
                            Ok(msg) => {
                                // reset local PoW state
                                let mut client_network_info =
                                    self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                                client_network_info.local_pow = false;
                                msg
                            }
                            Err(e) => {
                                // reset local PoW state
                                let mut client_network_info =
                                    self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                                client_network_info.local_pow = false;
                                return Err(e);
                            }
                        };
                        let message_dto = MessageDto::from(&message_with_local_pow);

                        self.node_manager
                            .post_request_json(path, timeout, serde_json::to_value(message_dto)?, true)
                            .await?
                    } else {
                        return Err(Error::NodeError(e));
                    }
                } else {
                    return Err(e);
                }
            }
        };

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
        let path = &format!(
            "api/v1/outputs/{}{}",
            output_id.output_id().transaction_id(),
            hex::encode(output_id.output_id().index().to_le_bytes())
        );

        let resp: SuccessBody<OutputResponse> = self
            .node_manager
            .get_request(path, None, self.get_timeout(Api::GetOutput))
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
            let address_outputs = self.get_address().outputs(address, Default::default()).await?;
            for output in address_outputs.iter() {
                output_to_query.insert(output.to_owned());
            }
            // 1000 is the max amount of outputs we get from the node, so if we reach that limit we maybe don't get all
            // outputs and that's why we additionally only request dust allowance outputs
            if address_outputs.len() == RESPONSE_MAX_OUTPUTS {
                let address_dust_allowance_outputs = self
                    .get_address()
                    .outputs(
                        address,
                        OutputsOptions {
                            include_spent: false,
                            output_type: Some(OutputType::SignatureLockedDustAllowance),
                        },
                    )
                    .await?;
                for output in address_dust_allowance_outputs.iter() {
                    output_to_query.insert(output.to_owned());
                }
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
        let path = &format!("api/v1/milestones/{index}");

        let resp: SuccessBody<MilestoneResponseDto> = self
            .node_manager
            .get_request(path, None, self.get_timeout(Api::GetMilestone))
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
        let path = &format!("api/v1/milestones/{index}/utxo-changes");

        let resp: SuccessBody<MilestoneUTXOChanges> = self
            .node_manager
            .get_request(path, None, self.get_timeout(Api::GetMilestone))
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/receipts endpoint
    /// Get all receipts.
    pub async fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let path = &"api/v1/receipts";

        let resp: SuccessBody<ReceiptsResponse> = self.node_manager.get_request(path, None, GET_API_TIMEOUT).await?;

        Ok(resp.data.receipts)
    }

    /// GET /api/v1/receipts/{migratedAt} endpoint
    /// Get the receipts by the given milestone index.
    pub async fn get_receipts_migrated_at(&self, milestone_index: u32) -> Result<Vec<ReceiptDto>> {
        let path = &format!("api/v1/receipts/{milestone_index}");

        let resp: SuccessBody<ReceiptsResponse> = self.node_manager.get_request(path, None, GET_API_TIMEOUT).await?;

        Ok(resp.data.receipts)
    }

    /// GET /api/v1/treasury endpoint
    /// Get the treasury output.
    pub async fn get_treasury(&self) -> Result<TreasuryResponse> {
        let path = "api/v1/treasury";

        let resp: SuccessBody<TreasuryResponse> = self.node_manager.get_request(path, None, GET_API_TIMEOUT).await?;

        Ok(resp.data)
    }

    /// GET /api/v1/transactions/{transactionId}/included-message
    /// Returns the included message of the transaction.
    pub async fn get_included_message(&self, transaction_id: &TransactionId) -> Result<Message> {
        let path = &format!("api/v1/transactions/{transaction_id}/included-message");

        let resp: SuccessBody<MessageResponse> = self.node_manager.get_request(path, None, GET_API_TIMEOUT).await?;
        Ok(Message::try_from(&resp.data.0)?)
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
        let reattach_message = {
            #[cfg(feature = "wasm")]
            {
                let network_id = self.get_network_id().await?;
                let mut tips = self.get_tips().await?;
                tips.sort_unstable_by_key(|a| a.pack_new());
                tips.dedup();
                let mut message_builder = MessageBuilder::<ClientMiner>::new()
                    .with_network_id(network_id)
                    .with_parents(Parents::new(tips)?);
                if let Some(p) = message.payload().to_owned() {
                    message_builder = message_builder.with_payload(p)
                }
                message_builder.finish().map_err(Error::MessageError)?
            }
            #[cfg(not(feature = "wasm"))]
            {
                finish_pow(self, message.payload().to_owned()).await?
            }
        };

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
        tips.push(*message_id);
        // Sort tips/parents
        tips.sort_unstable_by_key(|a| a.pack_new());
        tips.dedup();

        let promote_message = MessageBuilder::<ClientMiner>::new()
            .with_network_id(network_id)
            .with_parents(Parents::new(tips)?)
            .with_nonce_provider(self.get_pow_provider().await, min_pow_score)
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
        GetAddressesBuilder::new(seed).with_client(self)
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

    /// Return the balance for a provided seed
    /// Addresses with balance must be consecutive, so this method will return once it encounters a zero
    /// balance address.
    pub fn get_balance<'a>(&'a self, seed: &'a Seed) -> GetBalanceBuilder<'a> {
        GetBalanceBuilder::new(self, seed)
    }

    /// Return the balance in iota for the given addresses; No seed needed to do this since we are only checking and
    /// already know the addresses.
    pub async fn get_address_balances(&self, addresses: &[String]) -> Result<Vec<BalanceAddressResponse>> {
        let mut address_balance_pairs = Vec::new();
        for address in addresses {
            let balance_response = self.get_address().balance(address).await?;
            address_balance_pairs.push(balance_response);
        }
        Ok(address_balance_pairs)
    }

    /// Transforms bech32 to hex
    pub fn bech32_to_hex(bech32: &str) -> crate::Result<String> {
        let address = Address::try_from_bech32(bech32)?;
        let Address::Ed25519(ed) = address;
        Ok(ed.to_string())
    }

    /// Transforms a hex encoded address to a bech32 encoded address
    pub async fn hex_to_bech32(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        let address: Ed25519Address = hex.parse::<Ed25519Address>()?;
        match bech32_hrp {
            Some(hrp) => Ok(Address::Ed25519(address).to_bech32(hrp)),
            None => Ok(Address::Ed25519(address).to_bech32(self.get_bech32_hrp().await?.as_str())),
        }
    }

    /// Transforms a hex encoded public key to a bech32 encoded address
    pub async fn hex_public_key_to_bech32_address(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        let mut public_key = [0u8; ED25519_ADDRESS_LENGTH];
        hex::decode_to_slice(hex, &mut public_key)?;

        let address = Blake2b256::digest(&public_key)
            .try_into()
            .map_err(|_e| Error::Blake2b256Error("Hashing the public key failed."))?;
        let address: Ed25519Address = Ed25519Address::new(address);
        match bech32_hrp {
            Some(hrp) => Ok(Address::Ed25519(address).to_bech32(hrp)),
            None => Ok(Address::Ed25519(address).to_bech32(self.get_bech32_hrp().await?.as_str())),
        }
    }

    /// Returns a valid Address parsed from a String.
    pub fn parse_bech32_address(address: &str) -> crate::Result<Address> {
        Ok(Address::try_from_bech32(address)?)
    }

    /// Checks if a String is a valid bech32 encoded address.
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
    /// milestone). Default interval is 5 seconds and max attempts is 40. Returns the included message at first position
    /// and additional reattached messages
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
        for _ in 0..max_attempts.unwrap_or(40) {
            #[cfg(feature = "wasm")]
            {
                TimeoutFuture::new((interval.unwrap_or(5) * 1000).try_into().unwrap()).await;
            }
            #[cfg(not(feature = "wasm"))]
            sleep(Duration::from_secs(interval.unwrap_or(5))).await;
            // Check inclusion state for each attachment
            let message_ids_len = message_ids.len();
            let mut conflicting = false;
            for (index, msg_id) in message_ids.clone().iter().enumerate() {
                let message_metadata = self.get_message().metadata(msg_id).await?;
                if let Some(inclusion_state) = message_metadata.ledger_inclusion_state {
                    match inclusion_state {
                        LedgerInclusionStateDto::Included | LedgerInclusionStateDto::NoTransaction => {
                            // if original message, request it so we can return it on first position
                            if message_id == msg_id {
                                let mut included_and_reattached_messages =
                                    vec![(*message_id, self.get_message().data(message_id).await?)];
                                included_and_reattached_messages.extend(messages_with_id);
                                return Ok(included_and_reattached_messages);
                            } else {
                                // Move included message to first position
                                messages_with_id.rotate_left(index);
                                return Ok(messages_with_id);
                            }
                        }
                        // only set it as conflicting here and don't return, because another reattached message could
                        // have the included transaction
                        LedgerInclusionStateDto::Conflicting => conflicting = true,
                    };
                }
                // Only reattach or promote latest attachment of the message
                if index == message_ids_len - 1 {
                    if message_metadata.should_promote.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        self.promote_unchecked(message_ids.last().unwrap()).await?;
                    } else if message_metadata.should_reattach.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        let reattached = self.reattach_unchecked(message_ids.last().unwrap()).await?;
                        message_ids.push(reattached.0);
                        messages_with_id.push(reattached);
                    }
                }
            }
            // After we checked all our reattached messages, check if the transaction got reattached in another message
            // and confirmed
            if conflicting {
                let message = self.get_message().data(message_id).await?;
                if let Some(Payload::Transaction(transaction_payload)) = message.payload() {
                    let included_message = self.get_included_message(&transaction_payload.id()).await?;
                    let mut included_and_reattached_messages = vec![(included_message.id().0, included_message)];
                    included_and_reattached_messages.extend(messages_with_id);
                    return Ok(included_and_reattached_messages);
                }
            }
        }
        Err(Error::TangleInclusionError(message_id.to_string()))
    }

    /// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
    /// Returns the address to which the funds got consolidated, if any were available
    pub async fn consolidate_funds(
        &self,
        seed: &Seed,
        account_index: usize,
        address_range: Range<usize>,
    ) -> crate::Result<String> {
        crate::api::consolidate_funds(self, seed, account_index, address_range).await
    }
}

/// Hash the network id str from the nodeinfo to an u64 for the messageBuilder
pub fn hash_network(network_id_string: &str) -> Result<u64> {
    let bytes = Blake2b256::digest(network_id_string.as_bytes())[0..8]
        .try_into()
        .map_err(|_e| Error::Blake2b256Error("Hashing the network id failed."))?;

    Ok(u64::from_le_bytes(bytes))
}
