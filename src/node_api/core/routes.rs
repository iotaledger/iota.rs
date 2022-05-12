// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node core API routes.

use std::str::FromStr;

use bee_message::{
    output::OutputId,
    payload::{milestone::MilestoneId, transaction::TransactionId},
    Message, MessageDto, MessageId,
};
use bee_rest_api::types::{
    dtos::{PeerDto, ReceiptDto},
    responses::{
        MessageChildrenResponse, MessageMetadataResponse, MessageResponse, MilestoneResponse, OutputResponse,
        PeersResponse, ReceiptsResponse, SubmitMessageResponse, TipsResponse, TreasuryResponse, UtxoChangesResponse,
    },
};
use packable::PackableExt;

use crate::{constants::DEFAULT_API_TIMEOUT, Client, Error, NodeInfoWrapper, Result};

impl Client {
    // Node routes.

    /// Returns general information about the node.
    /// GET /api/v2/info
    pub async fn get_info(&self) -> Result<NodeInfoWrapper> {
        let path = "api/v2/info";

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    // Tangle routes.

    /// Returns tips that are ideal for attaching a message.
    /// GET /api/v2/tips
    pub async fn get_tips(&self) -> Result<Vec<MessageId>> {
        let path = "api/v2/tips";

        let resp = self
            .node_manager
            .get_request::<TipsResponse>(path, None, self.get_timeout(), false, false)
            .await?;

        resp.tip_message_ids
            .iter()
            .map(|tip| MessageId::from_str(tip).map_err(Error::MessageError))
            .collect::<Result<Vec<_>>>()
    }

    // Messages routes.

    /// Returns the MessageId of the submitted message.
    /// POST JSON to /api/v2/messages
    pub async fn post_message(&self, message: &Message) -> Result<MessageId> {
        let path = "api/v2/messages";
        let local_pow = self.get_local_pow().await;
        let timeout = if local_pow {
            self.get_timeout()
        } else {
            self.get_remote_pow_timeout()
        };
        let message_dto = MessageDto::from(message);

        // fallback to local PoW if remote PoW fails
        let resp = match self
            .node_manager
            .post_request_json::<SubmitMessageResponse>(path, timeout, serde_json::to_value(message_dto)?, local_pow)
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
                        #[cfg(not(target_family = "wasm"))]
                        let msg_res = crate::api::finish_pow(self, message.payload().cloned()).await;
                        #[cfg(target_family = "wasm")]
                        let msg_res = {
                            let min_pow_score = self.get_min_pow_score().await?;
                            let network_id = self.get_network_id().await?;
                            crate::api::finish_single_thread_pow(
                                client,
                                network_id,
                                None,
                                message.payload().cloned(),
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

        Ok(MessageId::from_str(&resp.message_id)?)
    }

    /// Returns the MessageId of the submitted message.
    /// POST /api/v2/messages
    pub async fn post_message_raw(&self, message: &Message) -> Result<MessageId> {
        let path = "api/v2/messages";
        let local_pow = self.get_local_pow().await;
        let timeout = if local_pow {
            self.get_timeout()
        } else {
            self.get_remote_pow_timeout()
        };

        // fallback to local PoW if remote PoW fails
        let resp = match self
            .node_manager
            .post_request_bytes::<SubmitMessageResponse>(path, timeout, &message.pack_to_vec(), local_pow)
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
                        #[cfg(not(target_family = "wasm"))]
                        let msg_res = crate::api::finish_pow(self, message.payload().cloned()).await;
                        #[cfg(target_family = "wasm")]
                        let msg_res = {
                            let min_pow_score = self.get_min_pow_score().await?;
                            let network_id = self.get_network_id().await?;
                            crate::api::finish_single_thread_pow(
                                client,
                                network_id,
                                None,
                                message.payload().cloned(),
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
                            .post_request_bytes(path, timeout, &message_with_local_pow.pack_to_vec(), true)
                            .await?
                    } else {
                        return Err(Error::NodeError(e));
                    }
                } else {
                    return Err(e);
                }
            }
        };

        Ok(MessageId::from_str(&resp.message_id)?)
    }

    /// Finds a message by its MessageId. This method returns the given message object.
    /// GET /api/v2/messages/{MessageId}
    pub async fn get_message(&self, message_id: &MessageId) -> Result<Message> {
        let path = &format!("api/v2/messages/{}", message_id);

        let resp = self
            .node_manager
            .get_request::<MessageResponse>(path, None, self.get_timeout(), false, true)
            .await?;

        Ok(Message::try_from(&resp.0)?)
    }

    /// Finds a message by its MessageId. This method returns the given message raw data.
    /// GET /api/v2/messages/{MessageId}
    pub async fn get_message_raw(&self, message_id: &MessageId) -> Result<Vec<u8>> {
        let path = &format!("api/v2/messages/{}", message_id);

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Returns the metadata of a message.
    /// GET /api/v2/messages/{MessageId}/metadata
    pub async fn get_message_metadata(&self, message_id: &MessageId) -> Result<MessageMetadataResponse> {
        let path = &format!("api/v2/messages/{}/metadata", message_id);

        self.node_manager
            .get_request(path, None, self.get_timeout(), true, true)
            .await
    }

    /// Returns the list of message IDs that reference a message by its identifier.
    /// GET /api/v2/messages/{messageID}/children
    pub async fn get_message_children(&self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        let path = &format!("api/v2/messages/{}/children", message_id);

        let resp = self
            .node_manager
            .get_request::<MessageChildrenResponse>(path, None, self.get_timeout(), false, true)
            .await?;

        resp.children_message_ids
            .iter()
            .map(|s| MessageId::from_str(s).map_err(Error::MessageError))
            .collect::<Result<Box<[MessageId]>>>()
    }

    // UTXO routes.

    /// Finds an output by its OutputId (TransactionId + output_index).
    /// GET /api/v2/outputs/{outputId}
    pub async fn get_output(&self, output_id: &OutputId) -> Result<OutputResponse> {
        let path = &format!("api/v2/outputs/{}", output_id);

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, true)
            .await
    }

    /// Gets all stored receipts.
    /// GET /api/v2/receipts
    pub async fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let path = &"api/v2/receipts";

        let resp = self
            .node_manager
            .get_request::<ReceiptsResponse>(path, None, DEFAULT_API_TIMEOUT, false, false)
            .await?;

        Ok(resp.receipts)
    }

    /// Gets the receipts by the given milestone index.
    /// GET /api/v2/receipts/{migratedAt}
    pub async fn get_receipts_migrated_at(&self, milestone_index: u32) -> Result<Vec<ReceiptDto>> {
        let path = &format!("api/v2/receipts/{}", milestone_index);

        let resp = self
            .node_manager
            .get_request::<ReceiptsResponse>(path, None, DEFAULT_API_TIMEOUT, false, false)
            .await?;

        Ok(resp.receipts)
    }

    /// Gets the current treasury output.
    /// The treasury output contains all tokens from the legacy network that have not yet been migrated.
    /// GET /api/v2/treasury
    pub async fn get_treasury(&self) -> Result<TreasuryResponse> {
        let path = "api/v2/treasury";

        self.node_manager
            .get_request(path, None, DEFAULT_API_TIMEOUT, false, false)
            .await
    }

    /// Returns the message that was included in the ledger for a given TransactionId.
    /// GET /api/v2/transactions/{transactionId}/included-message
    pub async fn get_included_message(&self, transaction_id: &TransactionId) -> Result<Message> {
        let path = &format!("api/v2/transactions/{}/included-message", transaction_id);

        let resp = self
            .node_manager
            .get_request::<MessageResponse>(path, None, self.get_timeout(), true, true)
            .await?;

        Ok(Message::try_from(&resp.0)?)
    }

    // Milestones routes.

    /// Gets the milestone by the given milestone id.
    /// GET /api/v2/milestones/{milestoneId}
    pub async fn get_milestone_by_id(&self, milestone_id: &MilestoneId) -> Result<MilestoneResponse> {
        let path = &format!("api/v2/milestones/{}", milestone_id);

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, true)
            .await
    }

    /// Gets the milestone by the given milestone id.
    /// GET /api/v2/milestones/{milestoneId}
    pub async fn get_milestone_by_id_raw(&self, milestone_id: &MilestoneId) -> Result<Vec<u8>> {
        let path = &format!("api/v2/milestones/{}", milestone_id);

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Gets all UTXO changes of a milestone by its milestone id.
    /// GET /api/v2/milestones/{milestoneId}/utxo-changes
    pub async fn get_utxo_changes_by_id(&self, milestone_id: &MilestoneId) -> Result<UtxoChangesResponse> {
        let path = &format!("api/v2/milestones/{}/utxo-changes", milestone_id);

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    /// Gets the milestone by the given milestone index.
    /// GET /api/v2/milestones/{index}
    pub async fn get_milestone_by_index(&self, index: u32) -> Result<MilestoneResponse> {
        let path = &format!("api/v2/milestones/by-index/{}", index);

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, true)
            .await
    }

    /// Gets the milestone by the given milestone index.
    /// GET /api/v2/milestones/{index}
    pub async fn get_milestone_by_index_raw(&self, index: u32) -> Result<Vec<u8>> {
        let path = &format!("api/v2/milestones/by-index/{}", index);

        self.node_manager
            .get_request_bytes(path, None, self.get_timeout())
            .await
    }

    /// Gets all UTXO changes of a milestone by its milestone index.
    /// GET /api/v2/milestones/by-index/{index}/utxo-changes
    pub async fn get_utxo_changes_by_index(&self, index: u32) -> Result<UtxoChangesResponse> {
        let path = &format!("api/v2/milestones/by-index/{}/utxo-changes", index);

        self.node_manager
            .get_request(path, None, self.get_timeout(), false, false)
            .await
    }

    // Peers routes.

    /// GET /api/v2/peers
    pub async fn get_peers(&self) -> Result<Vec<PeerDto>> {
        let path = "api/v2/peers";

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
