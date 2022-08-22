// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Client module to connect through HORNET or Bee with API usages

use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};

use bee_api_types::{
    dtos::LedgerInclusionStateDto,
    responses::{InfoResponse as NodeInfo, OutputResponse},
};
use bee_block::{
    address::Address,
    input::{Input, UtxoInput, INPUT_COUNT_MAX},
    output::{Output, OutputId, RentStructure, RentStructureBuilder},
    payload::{
        transaction::{TransactionEssence, TransactionId},
        Payload, TaggedDataPayload,
    },
    Block, BlockId,
};
use bee_pow::providers::{NonceProvider, NonceProviderBuilder};
use crypto::keys::slip10::Seed;
use url::Url;
#[cfg(feature = "mqtt")]
use {
    crate::node_api::mqtt::TopicEvent,
    crate::node_api::mqtt::{BrokerOptions, MqttEvent, MqttManager, TopicHandlerMap},
    crate::Topic,
    rumqttc::AsyncClient as MqttClient,
    tokio::sync::watch::{Receiver as WatchReceiver, Sender as WatchSender},
};
#[cfg(not(target_family = "wasm"))]
use {
    std::collections::HashMap,
    tokio::{
        runtime::Runtime,
        sync::broadcast::{Receiver, Sender},
        time::sleep,
    },
};

use crate::{
    api::{do_pow, ClientBlockBuilder, GetAddressesBuilder},
    builder::{ClientBuilder, NetworkInfo},
    constants::{
        DEFAULT_API_TIMEOUT, DEFAULT_RETRY_UNTIL_INCLUDED_INTERVAL, DEFAULT_RETRY_UNTIL_INCLUDED_MAX_AMOUNT,
        DEFAULT_TIPS_INTERVAL, FIVE_MINUTES_IN_SECONDS,
    },
    error::{Error, Result},
    node_api::{high_level::GetAddressBuilder, indexer::query_parameters::QueryParameter},
    node_manager::node::{Node, NodeAuth},
    secret::SecretManager,
    utils::{
        bech32_to_hex, generate_mnemonic, hash_network, hex_public_key_to_bech32_address, hex_to_bech32,
        is_address_valid, mnemonic_to_hex_seed, mnemonic_to_seed, parse_bech32_address,
    },
};

/// NodeInfo wrapper which contains the nodeinfo and the url from the node (useful when multiple nodes are used)
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfoWrapper {
    /// The returned nodeinfo
    #[serde(rename = "nodeInfo")]
    pub node_info: NodeInfo,
    /// The url from the node which returned the nodeinfo
    pub url: String,
}

/// An instance of the client using HORNET or Bee URI
// #[cfg_attr(target_family = "wasm", derive(Clone))]
#[derive(Clone)]
pub struct Client {
    #[allow(dead_code)]
    #[cfg(not(target_family = "wasm"))]
    pub(crate) runtime: Option<Arc<Runtime>>,
    /// Node manager
    pub(crate) node_manager: crate::node_manager::NodeManager,
    /// Flag to stop the node syncing
    #[cfg(not(target_family = "wasm"))]
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
    pub(crate) api_timeout: Duration,
    /// HTTP request timeout for remote PoW API call.
    pub(crate) remote_pow_timeout: Duration,
    #[allow(dead_code)] // not used for wasm
    /// pow_worker_count for local PoW.
    pub(crate) pow_worker_count: Option<usize>,
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
        #[cfg(not(target_family = "wasm"))]
        if let Some(sender) = self.sync_kill_sender.take() {
            sender.send(()).expect("failed to stop syncing process");
        }

        #[cfg(not(target_family = "wasm"))]
        if let Some(runtime) = self.runtime.take() {
            if let Ok(runtime) = Arc::try_unwrap(runtime) {
                runtime.shutdown_background();
            }
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
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn start_sync_process(
        runtime: &Runtime,
        sync: Arc<RwLock<HashSet<Node>>>,
        nodes: HashSet<Node>,
        node_sync_interval: Duration,
        network_info: Arc<RwLock<NetworkInfo>>,
        mut kill: Receiver<()>,
    ) {
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

    #[cfg(not(target_family = "wasm"))]
    pub(crate) async fn sync_nodes(
        sync: &Arc<RwLock<HashSet<Node>>>,
        nodes: &HashSet<Node>,
        network_info: &Arc<RwLock<NetworkInfo>>,
    ) {
        log::debug!("sync_nodes");
        let mut synced_nodes = HashSet::new();
        let mut network_nodes: HashMap<String, Vec<(NodeInfo, Node)>> = HashMap::new();
        for node in nodes {
            // Put the healthy node url into the network_nodes
            if let Ok(info) = Client::get_node_info(node.url.as_ref(), None).await {
                if info.status.is_healthy {
                    match network_nodes.get_mut(&info.protocol.network_name) {
                        Some(network_id_entry) => {
                            network_id_entry.push((info, node.clone()));
                        }
                        None => match &network_info
                            .read()
                            .map_or(NetworkInfo::default().network, |info| info.network.clone())
                        {
                            Some(id) => {
                                if info.protocol.network_name.contains(id) {
                                    network_nodes
                                        .insert(info.protocol.network_name.clone(), vec![(info, node.clone())]);
                                }
                            }
                            None => {
                                network_nodes.insert(info.protocol.network_name.clone(), vec![(info, node.clone())]);
                            }
                        },
                    }
                } else {
                    log::debug!("{} is not healthy: {:?}", node.url, info);
                }
            } else {
                log::error!("Couldn't get the nodeinfo from {}", node.url);
            }
        }
        // Get network_id with the most nodes
        let mut most_nodes = ("network_id", 0);
        for (network_id, node) in &network_nodes {
            if node.len() > most_nodes.1 {
                most_nodes.0 = network_id;
                most_nodes.1 = node.len();
            }
        }
        if let Some(nodes) = network_nodes.get(most_nodes.0) {
            for (info, node_url) in nodes.iter() {
                if let Ok(mut client_network_info) = network_info.write() {
                    client_network_info.network_id = hash_network(&info.protocol.network_name).ok();
                    client_network_info.min_pow_score = Some(info.protocol.min_pow_score);
                    client_network_info.bech32_hrp = Some(info.protocol.bech32_hrp.clone());
                    client_network_info.rent_structure = Some(info.protocol.rent_structure.clone());
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

    /// Gets the miner to use based on the Pow setting
    pub async fn get_pow_provider(&self) -> impl NonceProvider {
        let local_pow: bool = self.get_local_pow().await;
        #[cfg(target_family = "wasm")]
        let miner = crate::api::wasm_miner::SingleThreadedMiner::builder()
            .local_pow(local_pow)
            .finish();
        #[cfg(not(target_family = "wasm"))]
        let miner = {
            let mut miner = crate::api::miner::ClientMiner::builder().with_local_pow(local_pow);
            if let Some(worker_count) = self.pow_worker_count {
                miner = miner.with_worker_count(worker_count)
            }
            miner.finish()
        };
        miner
    }

    /// Gets the network related information such as network_id and min_pow_score
    /// and if it's the default one, sync it first and set the NetworkInfo.
    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        let not_synced = self.network_info.read().map_or(true, |info| info.network_id.is_none());

        // For WASM we don't have the node syncing process, which updates the network_info every 60 seconds, but the Pow
        // difficulty or the byte cost could change via a milestone, so we request the nodeinfo every time, so we don't
        // create invalid transactions/blocks
        if not_synced || cfg!(target_family = "wasm") {
            let info = self.get_info().await?.node_info;
            let network_id = hash_network(&info.protocol.network_name).ok();
            {
                let mut client_network_info = self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                client_network_info.bech32_hrp = Some(info.protocol.bech32_hrp);
                client_network_info.min_pow_score = Some(info.protocol.min_pow_score);
                client_network_info.network_id = network_id;
                client_network_info.rent_structure = Some(info.protocol.rent_structure);
            }
        }
        let res = self
            .network_info
            .read()
            .map_or(NetworkInfo::default(), |info| info.clone());
        Ok(res)
    }

    /// Gets the network id of the node we're connecting to.
    pub async fn get_network_id(&self) -> Result<u64> {
        let network_info = self.get_network_info().await?;
        network_info
            .network_id
            .ok_or(Error::MissingParameter("Missing network id."))
    }

    /// returns the bech32_hrp
    pub async fn get_bech32_hrp(&self) -> Result<String> {
        self.get_network_info()
            .await?
            .bech32_hrp
            .ok_or(Error::MissingParameter("Missing bech32_hrp."))
    }

    /// returns the min pow score
    pub async fn get_min_pow_score(&self) -> Result<f64> {
        self.get_network_info()
            .await?
            .min_pow_score
            .ok_or(Error::MissingParameter("Missing min_pow_score."))
    }

    /// returns the tips interval
    pub async fn get_tips_interval(&self) -> u64 {
        self.network_info
            .read()
            .map_or(DEFAULT_TIPS_INTERVAL, |info| info.tips_interval)
    }

    /// returns if local pow should be used or not
    pub async fn get_local_pow(&self) -> bool {
        self.network_info
            .read()
            .map_or(NetworkInfo::default().local_pow, |info| info.local_pow)
    }

    /// returns the rent structure
    pub async fn get_rent_structure(&self) -> Result<RentStructure> {
        let rent_structure = self
            .get_network_info()
            .await?
            .rent_structure
            .ok_or(Error::MissingParameter("Missing rent_structure."))?;

        let rent_structure = RentStructureBuilder::new()
            .byte_cost(rent_structure.v_byte_cost)
            .key_factor(rent_structure.v_byte_factor_key)
            .data_factor(rent_structure.v_byte_factor_data)
            .finish();
        Ok(rent_structure)
    }

    pub(crate) fn get_timeout(&self) -> Duration {
        self.api_timeout
    }

    pub(crate) fn get_remote_pow_timeout(&self) -> Duration {
        self.remote_pow_timeout
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
    #[cfg(not(target_family = "wasm"))]
    pub async fn unsynced_nodes(&self) -> HashSet<&Node> {
        self.node_manager.synced_nodes.read().map_or(HashSet::new(), |synced| {
            self.node_manager
                .nodes
                .iter()
                .filter(|node| !synced.contains(node))
                .collect()
        })
    }

    ///////////////////////////////////////////////////////////////////////
    // MQTT API
    //////////////////////////////////////////////////////////////////////

    /// Returns a handle to the MQTT topics manager.
    #[cfg(feature = "mqtt")]
    pub fn subscriber(&mut self) -> MqttManager<'_> {
        MqttManager::new(self)
    }

    /// Subscribe to MQTT events with a callback.
    #[cfg(feature = "mqtt")]
    pub async fn subscribe<C: Fn(&TopicEvent) + Send + Sync + 'static>(
        &mut self,
        topics: Vec<Topic>,
        callback: C,
    ) -> crate::Result<()> {
        MqttManager::new(self).with_topics(topics).subscribe(callback).await
    }

    /// Unsubscribe from MQTT events.
    #[cfg(feature = "mqtt")]
    pub async fn unsubscribe(&mut self, topics: Vec<Topic>) -> crate::Result<()> {
        MqttManager::new(self).with_topics(topics).unsubscribe().await
    }

    /// Returns the mqtt event receiver.
    #[cfg(feature = "mqtt")]
    pub fn mqtt_event_receiver(&self) -> WatchReceiver<MqttEvent> {
        self.mqtt_event_channel.1.clone()
    }

    //////////////////////////////////////////////////////////////////////
    // Node core API
    //////////////////////////////////////////////////////////////////////

    /// GET /api/core/v2/info endpoint
    pub async fn get_node_info(url: &str, auth: Option<NodeAuth>) -> Result<NodeInfo> {
        let mut url = crate::node_manager::builder::validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name)
                    .map_err(|_| crate::Error::UrlAuthError("username".to_string()))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::Error::UrlAuthError("password".to_string()))?;
            }
        }
        let path = "api/core/v2/info";
        url.set_path(path);

        let resp: NodeInfo = crate::node_manager::http_client::HttpClient::new()
            .get(
                Node {
                    url,
                    auth,
                    disabled: false,
                },
                DEFAULT_API_TIMEOUT,
            )
            .await?
            .into_json()
            .await?;

        Ok(resp)
    }

    /// GET /api/indexer/v1/outputs/basic{query} endpoint
    pub fn get_address(&self) -> GetAddressBuilder<'_> {
        GetAddressBuilder::new(self)
    }

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////

    /// Get the inputs of a transaction for the given transaction id.
    pub async fn inputs_from_transaction_id(&self, transaction_id: &TransactionId) -> Result<Vec<OutputResponse>> {
        let block = self.get_included_block(transaction_id).await?;

        let inputs = match block.payload() {
            Some(Payload::Transaction(t)) => match t.essence() {
                TransactionEssence::Regular(e) => e.inputs(),
            },
            _ => {
                unreachable!()
            }
        };

        let input_ids = inputs
            .iter()
            .map(|i| match i {
                Input::Utxo(input) => *input.output_id(),
                Input::Treasury(_) => {
                    unreachable!()
                }
            })
            .collect();

        self.get_outputs(input_ids).await
    }

    /// A generic send function for easily sending transaction or tagged data blocks.
    pub fn block(&self) -> ClientBlockBuilder<'_> {
        ClientBlockBuilder::new(self)
    }

    /// Return a list of addresses from a secret manager regardless of their validity.
    pub fn get_addresses<'a>(&'a self, secret_manager: &'a SecretManager) -> GetAddressesBuilder<'a> {
        GetAddressesBuilder::new(secret_manager).with_client(self)
    }

    /// Find all blocks by provided block IDs.
    pub async fn find_blocks(&self, block_ids: &[BlockId]) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();

        // Use a `HashSet` to prevent duplicate block_ids.
        let mut block_ids_to_query = HashSet::<BlockId>::new();

        // Collect the `BlockId` in the HashSet.
        for block_id in block_ids {
            block_ids_to_query.insert(*block_id);
        }

        // Use `get_block()` API to get the `Block`.
        for block_id in block_ids_to_query {
            let block = self.get_block(&block_id).await?;
            blocks.push(block);
        }
        Ok(blocks)
    }

    /// Retries (promotes or reattaches) a block for provided block id. Block should only be
    /// retried only if they are valid and haven't been confirmed for a while.
    pub async fn retry(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        // Get the metadata to check if it needs to promote or reattach
        let block_metadata = self.get_block_metadata(block_id).await?;
        if block_metadata.should_promote.unwrap_or(false) {
            self.promote_unchecked(block_id).await
        } else if block_metadata.should_reattach.unwrap_or(false) {
            self.reattach_unchecked(block_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(block_id.to_string()))
        }
    }

    /// Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
    /// milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first position
    /// and additional reattached blocks
    pub async fn retry_until_included(
        &self,
        block_id: &BlockId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> Result<Vec<(BlockId, Block)>> {
        log::debug!("[retry_until_included]");
        // Attachments of the Block to check inclusion state
        let mut block_ids = vec![*block_id];
        // Reattached Blocks that get returned
        let mut blocks_with_id = Vec::new();
        for _ in 0..max_attempts.unwrap_or(DEFAULT_RETRY_UNTIL_INCLUDED_MAX_AMOUNT) {
            #[cfg(target_family = "wasm")]
            {
                gloo_timers::future::TimeoutFuture::new(
                    (interval.unwrap_or(DEFAULT_RETRY_UNTIL_INCLUDED_INTERVAL) * 1000)
                        .try_into()
                        .unwrap(),
                )
                .await;
            }
            #[cfg(not(target_family = "wasm"))]
            sleep(Duration::from_secs(
                interval.unwrap_or(DEFAULT_RETRY_UNTIL_INCLUDED_INTERVAL),
            ))
            .await;
            // Check inclusion state for each attachment
            let block_ids_len = block_ids.len();
            let mut conflicting = false;
            for (index, block_id_) in block_ids.clone().iter().enumerate() {
                let block_metadata = self.get_block_metadata(block_id_).await?;
                if let Some(inclusion_state) = block_metadata.ledger_inclusion_state {
                    match inclusion_state {
                        LedgerInclusionStateDto::Included | LedgerInclusionStateDto::NoTransaction => {
                            // if original block, request it so we can return it on first position
                            if block_id == block_id_ {
                                let mut included_and_reattached_blocks =
                                    vec![(*block_id, self.get_block(block_id).await?)];
                                included_and_reattached_blocks.extend(blocks_with_id);
                                return Ok(included_and_reattached_blocks);
                            } else {
                                // Move included block to first position
                                blocks_with_id.rotate_left(index);
                                return Ok(blocks_with_id);
                            }
                        }
                        // only set it as conflicting here and don't return, because another reattached block could
                        // have the included transaction
                        LedgerInclusionStateDto::Conflicting => conflicting = true,
                    };
                }
                // Only reattach or promote latest attachment of the block
                if index == block_ids_len - 1 {
                    if block_metadata.should_promote.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        self.promote_unchecked(block_ids.last().unwrap()).await?;
                    } else if block_metadata.should_reattach.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        let reattached = self.reattach_unchecked(block_ids.last().unwrap()).await?;
                        block_ids.push(reattached.0);
                        blocks_with_id.push(reattached);
                    }
                }
            }
            // After we checked all our reattached blocks, check if the transaction got reattached in another block
            // and confirmed
            if conflicting {
                let block = self.get_block(block_id).await?;
                if let Some(Payload::Transaction(transaction_payload)) = block.payload() {
                    let included_block = self.get_included_block(&transaction_payload.id()).await?;
                    let mut included_and_reattached_blocks = vec![(included_block.id(), included_block)];
                    included_and_reattached_blocks.extend(blocks_with_id);
                    return Ok(included_and_reattached_blocks);
                }
            }
        }
        Err(Error::TangleInclusionError(block_id.to_string()))
    }

    /// Function to find inputs from addresses for a provided amount (useful for offline signing), ignoring outputs with
    /// additional unlock conditions
    pub async fn find_inputs(&self, addresses: Vec<String>, amount: u64) -> Result<Vec<UtxoInput>> {
        // Get outputs from node and select inputs
        let mut available_outputs = Vec::new();

        for address in addresses {
            let basic_output_ids = self
                .basic_output_ids(vec![
                    QueryParameter::Address(address.to_string()),
                    QueryParameter::HasExpiration(false),
                    QueryParameter::HasTimelock(false),
                    QueryParameter::HasStorageDepositReturn(false),
                ])
                .await?;

            available_outputs.extend(self.get_outputs(basic_output_ids).await?);
        }

        let mut basic_outputs = Vec::new();
        let current_time = self.get_time_checked().await?;

        for output_resp in available_outputs {
            let (amount, _) = ClientBlockBuilder::get_output_amount_and_address(
                &Output::try_from(&output_resp.output)?,
                None,
                current_time,
            )?;
            basic_outputs.push((
                UtxoInput::new(
                    TransactionId::from_str(&output_resp.metadata.transaction_id)?,
                    output_resp.metadata.output_index,
                )?,
                amount,
            ));
        }
        basic_outputs.sort_by(|l, r| r.1.cmp(&l.1));

        let mut total_already_spent = 0;
        let mut selected_inputs = Vec::new();
        for (_offset, output_wrapper) in basic_outputs
            .into_iter()
            // Max inputs is 128
            .take(INPUT_COUNT_MAX.into())
            .enumerate()
        {
            // Break if we have enough funds and don't create dust for the remainder
            if total_already_spent == amount || total_already_spent >= amount {
                break;
            }
            selected_inputs.push(output_wrapper.0.clone());
            total_already_spent += output_wrapper.1;
        }

        if total_already_spent < amount {
            return Err(crate::Error::NotEnoughBalance {
                found: total_already_spent,
                required: amount,
            });
        }

        Ok(selected_inputs)
    }

    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceeds individual node limit.
    pub async fn find_outputs(&self, output_ids: &[OutputId], addresses: &[String]) -> Result<Vec<OutputResponse>> {
        let mut output_responses = self.get_outputs(output_ids.to_vec()).await?;

        // Use `get_address()` API to get the address outputs first,
        // then collect the `UtxoInput` in the HashSet.
        for address in addresses {
            // Get output ids of outputs that can be controlled by this address without further unlock constraints
            let basic_output_ids = self
                .basic_output_ids(vec![
                    QueryParameter::Address(address.to_string()),
                    QueryParameter::HasExpiration(false),
                    QueryParameter::HasTimelock(false),
                    QueryParameter::HasStorageDepositReturn(false),
                ])
                .await?;

            output_responses.extend(self.get_outputs(basic_output_ids).await?);
        }

        Ok(output_responses.clone())
    }

    /// Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub async fn reattach(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        let metadata = self.get_block_metadata(block_id).await?;
        if metadata.should_reattach.unwrap_or(false) {
            self.reattach_unchecked(block_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(block_id.to_string()))
        }
    }

    /// Reattach a block without checking if it should be reattached
    pub async fn reattach_unchecked(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        // Get the Block object by the BlockID.
        let block = self.get_block(block_id).await?;
        let reattach_block = {
            #[cfg(target_family = "wasm")]
            {
                crate::api::finish_single_threaded_pow(self, block.payload().cloned()).await?
            }
            #[cfg(not(target_family = "wasm"))]
            {
                crate::api::finish_multi_threaded_pow(self, block.payload().cloned()).await?
            }
        };

        // Post the modified
        let block_id = self.post_block_raw(&reattach_block).await?;
        // Get block if we use remote Pow, because the node will change parents and nonce
        let block = if self.get_local_pow().await {
            reattach_block
        } else {
            self.get_block(&block_id).await?
        };
        Ok((block_id, block))
    }

    /// Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
    /// method should error out and should not allow unnecessary promotions.
    pub async fn promote(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        let metadata = self.get_block_metadata(block_id).await?;
        if metadata.should_promote.unwrap_or(false) {
            self.promote_unchecked(block_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(block_id.to_string()))
        }
    }

    /// Promote a block without checking if it should be promoted
    pub async fn promote_unchecked(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        // Create a new block (zero value block) for which one tip would be the actual block.
        let mut tips = self.get_tips().await?;
        let min_pow_score = self.get_min_pow_score().await?;
        tips.push(*block_id);

        let miner = self.get_pow_provider().await;
        let promote_block = do_pow(miner, min_pow_score, None, tips).map_err(|_| Error::TransactionError)?;

        let block_id = self.post_block_raw(&promote_block).await?;
        // Get block if we use remote Pow, because the node will change parents and nonce.
        let block = if self.get_local_pow().await {
            promote_block
        } else {
            self.get_block(&block_id).await?
        };
        Ok((block_id, block))
    }

    /// Returns the local time checked with the timestamp of the latest milestone, if the difference is larger than 5
    /// minutes an error is returned to prevent locking outputs by accident for a wrong time.
    pub async fn get_time_checked(&self) -> Result<u32> {
        let current_time = {
            #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
            let now = instant::SystemTime::now().duration_since(instant::SystemTime::UNIX_EPOCH);
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);

            now.expect("Time went backwards").as_secs() as u32
        };

        let status_response = self.get_info().await?.node_info.status;
        let latest_ms_timestamp = status_response.latest_milestone.timestamp;
        // Check the local time is in the range of +-5 minutes of the node to prevent locking funds by accident
        if !(latest_ms_timestamp - FIVE_MINUTES_IN_SECONDS..latest_ms_timestamp + FIVE_MINUTES_IN_SECONDS)
            .contains(&current_time)
        {
            return Err(Error::TimeNotSynced {
                current_time,
                milestone_timestamp: latest_ms_timestamp,
            });
        }
        Ok(current_time)
    }

    //////////////////////////////////////////////////////////////////////
    // Utils
    //////////////////////////////////////////////////////////////////////

    /// Transforms bech32 to hex
    pub fn bech32_to_hex(bech32: &str) -> crate::Result<String> {
        bech32_to_hex(bech32)
    }

    /// Transforms a hex encoded address to a bech32 encoded address
    pub async fn hex_to_bech32(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        let bech32_hrp = match bech32_hrp {
            Some(hrp) => hrp.into(),
            None => self.get_bech32_hrp().await?,
        };
        hex_to_bech32(hex, &bech32_hrp)
    }

    /// Transforms a hex encoded public key to a bech32 encoded address
    pub async fn hex_public_key_to_bech32_address(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        let bech32_hrp = match bech32_hrp {
            Some(hrp) => hrp.into(),
            None => self.get_bech32_hrp().await?,
        };
        hex_public_key_to_bech32_address(hex, &bech32_hrp)
    }

    /// Returns a valid Address parsed from a String.
    pub fn parse_bech32_address(address: &str) -> crate::Result<Address> {
        parse_bech32_address(address)
    }

    /// Checks if a String is a valid bech32 encoded address.
    #[must_use]
    pub fn is_address_valid(address: &str) -> bool {
        is_address_valid(address)
    }

    /// Generates a new mnemonic.
    pub fn generate_mnemonic() -> Result<String> {
        generate_mnemonic()
    }

    /// Returns a seed for a mnemonic.
    pub fn mnemonic_to_seed(mnemonic: &str) -> Result<Seed> {
        mnemonic_to_seed(mnemonic)
    }

    /// Returns a hex encoded seed for a mnemonic.
    pub fn mnemonic_to_hex_seed(mnemonic: &str) -> Result<String> {
        mnemonic_to_hex_seed(mnemonic)
    }

    /// UTF-8 encodes the `tag` of a given TaggedDataPayload.
    pub fn tag_to_utf8(payload: &TaggedDataPayload) -> Result<String> {
        String::from_utf8(payload.tag().to_vec()).map_err(|_| Error::TaggedDataError("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes the `data` of a given TaggedDataPayload.
    pub fn data_to_utf8(payload: &TaggedDataPayload) -> Result<String> {
        String::from_utf8(payload.data().to_vec())
            .map_err(|_| Error::TaggedDataError("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes both the `tag` and `data` of a given TaggedDataPayload.
    pub fn tagged_data_to_utf8(payload: &TaggedDataPayload) -> Result<(String, String)> {
        Ok((Client::tag_to_utf8(payload)?, Client::data_to_utf8(payload)?))
    }
}
