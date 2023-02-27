// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node core API routes.

use std::str::FromStr;

use iota_types::{
    api::core::{
        dto::{PeerDto, ReceiptDto},
        response::{
            BlockMetadataResponse, BlockResponse, InfoResponse, MilestoneResponse, OutputWithMetadataResponse,
            PeersResponse, ReceiptsResponse, RoutesResponse, SubmitBlockResponse, TipsResponse, TreasuryResponse,
            UtxoChangesResponse,
        },
    },
    block::{
        output::{dto::OutputMetadataDto, OutputId},
        payload::{
            milestone::{MilestoneId, MilestonePayload},
            transaction::TransactionId,
        },
        Block, BlockDto, BlockId,
    },
};
use packable::PackableExt;
use url::Url;

use crate::{
    constants::{DEFAULT_API_TIMEOUT, DEFAULT_USER_AGENT},
    node_manager::node::{Node, NodeAuth},
    Client, Error, Result,
};

/// NodeInfo wrapper which contains the node info and the url from the node (useful when multiple nodes are used)
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfoWrapper {
    /// The returned node info
    #[serde(rename = "nodeInfo")]
    pub node_info: InfoResponse,
    /// The url from the node which returned the node info
    pub url: String,
}

impl Client {
    // Node routes.

    /// Returns the health of the node.
    /// GET /health
    pub async fn get_health(&self, url: &str) -> Result<bool> {
        let path = "health";

        let mut url = Url::parse(url)?;
        url.set_path(path);
        let status = crate::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string())
            .get(
                Node {
                    url,
                    auth: None,
                    disabled: false,
                },
                DEFAULT_API_TIMEOUT,
            )
            .await?
            .status();

        match status {
            200 => Ok(true),
            _ => Ok(false),
        }
    }

    /// Returns the available API route groups of the node.
    /// GET /api/routes
    pub async fn get_routes(&self) -> Result<RoutesResponse> {
        let path = "api/routes";

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    /// Returns general information about the node.
    /// GET /api/core/v2/info
    pub async fn get_info(&self) -> Result<NodeInfoWrapper> {
        let path = "api/core/v2/info";

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    /// GET /api/core/v2/info endpoint
    pub async fn get_node_info(url: &str, auth: Option<NodeAuth>) -> Result<InfoResponse> {
        let mut url = crate::node_manager::builder::validate_url(Url::parse(url)?)?;
        if let Some(auth) = &auth {
            if let Some((name, password)) = &auth.basic_auth_name_pwd {
                url.set_username(name).map_err(|_| crate::Error::UrlAuth("username"))?;
                url.set_password(Some(password))
                    .map_err(|_| crate::Error::UrlAuth("password"))?;
            }
        }
        let path = "api/core/v2/info";
        url.set_path(path);

        let resp: InfoResponse = crate::node_manager::http_client::HttpClient::new(DEFAULT_USER_AGENT.to_string())
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

    // Tangle routes.

    /// Returns tips that are ideal for attaching a block.
    /// GET /api/core/v2/tips
    pub async fn get_tips(&self) -> Result<Vec<BlockId>> {
        let path = "api/core/v2/tips";

        let resp = self
            .node_manager
            .get_request::<TipsResponse>(path, None, self.get_timeout(), false, false)
            .await?;

        resp.tips
            .iter()
            .map(|tip| BlockId::from_str(tip).map_err(Error::Block))
            .collect::<Result<Vec<_>>>()
    }

    // Blocks routes.

    /// Returns the BlockId of the submitted block.
    /// POST JSON to /api/core/v2/blocks
    pub async fn post_block(&self, block: &Block) -> Result<BlockId> {
        let path = "api/core/v2/blocks";
        let local_pow = self.get_local_pow();
        let timeout = if local_pow {
            self.get_timeout()
        } else {
            self.get_remote_pow_timeout()
        };
        let block_dto = BlockDto::from(block);

        // fallback to local PoW if remote PoW fails
        let resp = match self
            .node_manager
            .post_request_json::<SubmitBlockResponse>(path, timeout, serde_json::to_value(block_dto)?, local_pow)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                if let Error::Node(e) = e {
                    let fallback_to_local_pow = self.get_fallback_to_local_pow();
                    // hornet and bee return different error blocks
                    if (e == *"No available nodes with remote Pow"
                        || e.contains("proof of work is not enabled")
                        || e.contains("`Pow` not enabled"))
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
                        let block_res = self.finish_block_builder(None, block.payload().cloned()).await;
                        let block_with_local_pow = match block_res {
                            Ok(block) => {
                                // reset local PoW state
                                let mut client_network_info =
                                    self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                                client_network_info.local_pow = false;
                                block
                            }
                            Err(e) => {
                                // reset local PoW state
                                self.network_info
                                    .write()
                                    .map_err(|_| crate::Error::PoisonError)?
                                    .local_pow = false;
                                return Err(e);
                            }
                        };
                        let block_dto = BlockDto::from(&block_with_local_pow);

                        self.node_manager
                            .post_request_json(path, timeout, serde_json::to_value(block_dto)?, true)
                            .await?
                    } else {
                        return Err(Error::Node(e));
                    }
                } else {
                    return Err(e);
                }
            }
        };

        Ok(BlockId::from_str(&resp.block_id)?)
    }

    /// Returns the BlockId of the submitted block.
    /// POST /api/core/v2/blocks
    pub async fn post_block_raw(&self, block: &Block) -> Result<BlockId> {
        let path = "api/core/v2/blocks";
        let local_pow = self.get_local_pow();
        let timeout = if local_pow {
            self.get_timeout()
        } else {
            self.get_remote_pow_timeout()
        };

        // fallback to local Pow if remote Pow fails
        let resp = match self
            .node_manager
            .post_request_bytes::<SubmitBlockResponse>(path, timeout, &block.pack_to_vec(), local_pow)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                if let Error::Node(e) = e {
                    let fallback_to_local_pow = self.get_fallback_to_local_pow();
                    // hornet and bee return different error blocks
                    if (e == *"No available nodes with remote Pow"
                        || e.contains("proof of work is not enabled")
                        || e.contains("`Pow` not enabled"))
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
                        let block_res = self.finish_block_builder(None, block.payload().cloned()).await;
                        let block_with_local_pow = match block_res {
                            Ok(block) => {
                                // reset local PoW state
                                let mut client_network_info =
                                    self.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                                client_network_info.local_pow = false;
                                block
                            }
                            Err(e) => {
                                // reset local PoW state
                                self.network_info
                                    .write()
                                    .map_err(|_| crate::Error::PoisonError)?
                                    .local_pow = false;
                                return Err(e);
                            }
                        };
                        self.node_manager
                            .post_request_bytes(path, timeout, &block_with_local_pow.pack_to_vec(), true)
                            .await?
                    } else {
                        return Err(Error::Node(e));
                    }
                } else {
                    return Err(e);
                }
            }
        };

        Ok(BlockId::from_str(&resp.block_id)?)
    }

    /// Finds a block by its BlockId. This method returns the given block object.
    /// GET /api/core/v2/blocks/{BlockId}
    pub async fn get_block(&self, block_id: &BlockId) -> Result<Block> {
        let path = &format!("api/core/v2/blocks/{block_id}");

        let resp = self
            .node_manager
            .get_request::<BlockResponse>(path, None, self.get_timeout(), false, true)
            .await?;

        match resp {
            BlockResponse::Json(dto) => Ok(Block::try_from_dto(&dto, &self.get_protocol_parameters().await?)?),
            BlockResponse::Raw(_) => Err(crate::Error::UnexpectedApiResponse),
        }
    }

    /// Finds a block by its BlockId. This method returns the given block raw data.
    /// GET /api/core/v2/blocks/{BlockId}
    pub async fn get_block_raw(&self, block_id: &BlockId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v2/blocks/{block_id}");

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Returns the metadata of a block.
    /// GET /api/core/v2/blocks/{BlockId}/metadata
    pub async fn get_block_metadata(&self, block_id: &BlockId) -> Result<BlockMetadataResponse> {
        let path = &format!("api/core/v2/blocks/{block_id}/metadata");

        self.node_manager
            .get_request(path, None, self.get_timeout(), true, true)
            .await
    }

    // UTXO routes.

    /// Finds an output, as JSON, by its OutputId (TransactionId + output_index).
    /// GET /api/core/v2/outputs/{outputId}
    pub async fn get_output(&self, output_id: &OutputId) -> Result<OutputWithMetadataResponse> {
        let path = &format!("api/core/v2/outputs/{output_id}");

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, true)
            .await
    }

    /// Finds an output, as raw bytes, by its OutputId (TransactionId + output_index).
    /// GET /api/core/v2/outputs/{outputId}
    pub async fn get_output_raw(&self, output_id: &OutputId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v2/outputs/{output_id}");

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Get the metadata for a given `OutputId` (TransactionId + output_index).
    /// GET /api/core/v2/outputs/{outputId}/metadata
    pub async fn get_output_metadata(&self, output_id: &OutputId) -> Result<OutputMetadataDto> {
        let path = &format!("api/core/v2/outputs/{output_id}/metadata");

        self.node_manager
            .get_request::<OutputMetadataDto>(path, None, self.get_timeout(), false, true)
            .await
    }

    /// Gets all stored receipts.
    /// GET /api/core/v2/receipts
    pub async fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let path = &"api/core/v2/receipts";

        let resp = self
            .node_manager
            .get_request::<ReceiptsResponse>(path, None, DEFAULT_API_TIMEOUT, false, false)
            .await?;

        Ok(resp.receipts)
    }

    /// Gets the receipts by the given milestone index.
    /// GET /api/core/v2/receipts/{migratedAt}
    pub async fn get_receipts_migrated_at(&self, milestone_index: u32) -> Result<Vec<ReceiptDto>> {
        let path = &format!("api/core/v2/receipts/{milestone_index}");

        let resp = self
            .node_manager
            .get_request::<ReceiptsResponse>(path, None, DEFAULT_API_TIMEOUT, false, false)
            .await?;

        Ok(resp.receipts)
    }

    /// Gets the current treasury output.
    /// The treasury output contains all tokens from the legacy network that have not yet been migrated.
    /// GET /api/core/v2/treasury
    pub async fn get_treasury(&self) -> Result<TreasuryResponse> {
        let path = "api/core/v2/treasury";

        self.node_manager
            .get_request(path, None, DEFAULT_API_TIMEOUT, false, false)
            .await
    }

    /// Returns the block, as object, that was included in the ledger for a given TransactionId.
    /// GET /api/core/v2/transactions/{transactionId}/included-block
    pub async fn get_included_block(&self, transaction_id: &TransactionId) -> Result<Block> {
        let path = &format!("api/core/v2/transactions/{transaction_id}/included-block");

        let resp = self
            .node_manager
            .get_request::<BlockResponse>(path, None, self.get_timeout(), true, true)
            .await?;

        match resp {
            BlockResponse::Json(dto) => Ok(Block::try_from_dto(&dto, &self.get_protocol_parameters().await?)?),
            BlockResponse::Raw(_) => Err(crate::Error::UnexpectedApiResponse),
        }
    }

    /// Returns the block, as raw bytes, that was included in the ledger for a given TransactionId.
    /// GET /api/core/v2/transactions/{transactionId}/included-block
    pub async fn get_included_block_raw(&self, transaction_id: &TransactionId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v2/transactions/{transaction_id}/included-block");

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Returns the metadata of the block that was included in the ledger for a given TransactionId.
    /// GET /api/core/v2/transactions/{transactionId}/included-block/metadata
    pub async fn get_included_block_metadata(&self, transaction_id: &TransactionId) -> Result<BlockMetadataResponse> {
        let path = &format!("api/core/v2/transactions/{transaction_id}/included-block/metadata");

        self.node_manager
            .get_request(path, None, self.get_timeout(), true, true)
            .await
    }

    // Milestones routes.

    /// Gets the milestone by the given milestone id.
    /// GET /api/core/v2/milestones/{milestoneId}
    pub async fn get_milestone_by_id(&self, milestone_id: &MilestoneId) -> Result<MilestonePayload> {
        let path = &format!("api/core/v2/milestones/{milestone_id}");

        let resp = self
            .node_manager
            .get_request::<MilestoneResponse>(path, None, self.get_timeout(), false, true)
            .await?;

        match resp {
            MilestoneResponse::Json(dto) => Ok(MilestonePayload::try_from_dto(
                &dto,
                &self.get_protocol_parameters().await?,
            )?),
            MilestoneResponse::Raw(_) => Err(crate::Error::UnexpectedApiResponse),
        }
    }

    /// Gets the milestone by the given milestone id.
    /// GET /api/core/v2/milestones/{milestoneId}
    pub async fn get_milestone_by_id_raw(&self, milestone_id: &MilestoneId) -> Result<Vec<u8>> {
        let path = &format!("api/core/v2/milestones/{milestone_id}");

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Gets all UTXO changes of a milestone by its milestone id.
    /// GET /api/core/v2/milestones/{milestoneId}/utxo-changes
    pub async fn get_utxo_changes_by_id(&self, milestone_id: &MilestoneId) -> Result<UtxoChangesResponse> {
        let path = &format!("api/core/v2/milestones/{milestone_id}/utxo-changes");

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    /// Gets the milestone by the given milestone index.
    /// GET /api/core/v2/milestones/{index}
    pub async fn get_milestone_by_index(&self, index: u32) -> Result<MilestonePayload> {
        let path = &format!("api/core/v2/milestones/by-index/{index}");

        let resp = self
            .node_manager
            .get_request::<MilestoneResponse>(path, None, self.get_timeout(), false, true)
            .await?;

        match resp {
            MilestoneResponse::Json(dto) => Ok(MilestonePayload::try_from_dto(
                &dto,
                &self.get_protocol_parameters().await?,
            )?),
            MilestoneResponse::Raw(_) => Err(crate::Error::UnexpectedApiResponse),
        }
    }

    /// Gets the milestone by the given milestone index.
    /// GET /api/core/v2/milestones/{index}
    pub async fn get_milestone_by_index_raw(&self, index: u32) -> Result<Vec<u8>> {
        let path = &format!("api/core/v2/milestones/by-index/{index}");

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Gets all UTXO changes of a milestone by its milestone index.
    /// GET /api/core/v2/milestones/by-index/{index}/utxo-changes
    pub async fn get_utxo_changes_by_index(&self, index: u32) -> Result<UtxoChangesResponse> {
        let path = &format!("api/core/v2/milestones/by-index/{index}/utxo-changes");

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    // Peers routes.

    /// GET /api/core/v2/peers
    pub async fn get_peers(&self) -> Result<Vec<PeerDto>> {
        let path = "api/core/v2/peers";

        let resp = self
            .node_manager
            .get_request::<PeersResponse>(path, None, self.get_timeout(), false, false)
            .await?;

        Ok(resp.0)
    }

    // // RoutePeer is the route for getting peers by their peerID.
    // // GET returns the peer
    // // DELETE deletes the peer.
    // RoutePeer = "/peers/:" + restapipkg.ParameterPeerID

    // // RoutePeers is the route for getting all peers of the node.
    // // GET returns a list of all peers.
    // // POST adds a new peer.
    // RoutePeers = "/peers"

    // Control routes.

    // // RouteControlDatabasePrune is the control route to manually prune the database.
    // // POST prunes the database.
    // RouteControlDatabasePrune = "/control/database/prune"

    // // RouteControlSnapshotsCreate is the control route to manually create a snapshot files.
    // // POST creates a snapshot (full, delta or both).
    // RouteControlSnapshotsCreate = "/control/snapshots/create"
}
