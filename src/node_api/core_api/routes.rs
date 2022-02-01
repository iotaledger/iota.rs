// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API routes

use crate::{constants::DEFAULT_API_TIMEOUT, Client, Error, NodeInfoWrapper, Result};

use bee_message::{output::OutputId, payload::transaction::TransactionId, Message, MessageId};
use bee_rest_api::types::{
    body::SuccessBody,
    dtos::{MessageDto, PeerDto, ReceiptDto},
    responses::{
        MessageChildrenResponse, MessageMetadataResponse, MessageResponse, MilestoneResponse, OutputResponse,
        PeersResponse, ReceiptsResponse, SubmitMessageResponse, TipsResponse, TreasuryResponse, UtxoChangesResponse,
    },
};
use packable::PackableExt;

// https://github.com/gohornet/hornet/blob/stardust-utxo/plugins/restapi/v2/plugin.go

/// Returns general information about the node.
/// GET /api/v2/info endpoint
pub async fn get_info(client: &Client) -> Result<NodeInfoWrapper> {
    let path = "api/v2/info";

    let resp: NodeInfoWrapper = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    Ok(resp)
}

/// Returns non-lazy tips.
/// GET /api/v2/tips endpoint
pub async fn get_tips(client: &Client) -> Result<Vec<MessageId>> {
    let path = "api/v2/tips";

    let resp: SuccessBody<TipsResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    let mut tips = Vec::new();
    for tip in resp.data.tip_message_ids {
        let mut new_tip = [0u8; 32];
        hex::decode_to_slice(tip, &mut new_tip)?;
        tips.push(MessageId::from(new_tip));
    }
    Ok(tips)
}

/// Consume the builder and find a message by its MessageId. This method returns the given message object.
/// GET /api/v2/messages/{MessageId} endpoint
pub async fn data(client: &Client, message_id: &MessageId) -> Result<Message> {
    let path = &format!("api/v2/messages/{}", message_id);

    let resp: SuccessBody<MessageResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    Ok(Message::try_from(&resp.data.0)?)
}

/// Returns the metadata of a message.
/// GET /api/v2/messages/{MessageId}/metadata endpoint
pub async fn metadata(client: &Client, message_id: &MessageId) -> Result<MessageMetadataResponse> {
    let path = &format!("api/v2/messages/{}/metadata", message_id);

    let resp: SuccessBody<MessageMetadataResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    Ok(resp.data)
}

/// Consume the builder and find a message by its MessageId. This method returns the given message raw data.
/// GET /api/v2/messages/{MessageId}/raw endpoint
pub async fn raw(client: &Client, message_id: &MessageId) -> Result<String> {
    let path = &format!("api/v2/messages/{}/raw", message_id);
    let resp = client
        .node_manager
        .get_request_text(path, None, client.get_timeout())
        .await?;

    Ok(resp)
}

// // RouteMessageChildren is the route for getting message IDs of the children of a message, identified by its
// messageID. // GET returns the message IDs of all children.
// RouteMessageChildren = "/messages/:" + restapipkg.ParameterMessageID + "/children"

/// GET /api/v2/messages/{messageID}/children endpoint
/// Consume the builder and returns the list of message IDs that reference a message by its identifier.
pub async fn children(client: &Client, message_id: &MessageId) -> Result<Box<[MessageId]>> {
    let path = &format!("api/v2/messages/{}/children", message_id);

    let resp: SuccessBody<MessageChildrenResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    resp.data
        .children_message_ids
        .iter()
        .map(|s| {
            let mut message_id = [0u8; 32];
            hex::decode_to_slice(s, &mut message_id)?;
            Ok(MessageId::from(message_id))
        })
        .collect::<Result<Box<[MessageId]>>>()
}

/// Returns the MessageId of the submitted message.
/// POST /api/v2/messages endpoint
pub async fn post_message(client: &Client, message: &Message) -> Result<MessageId> {
    let path = "api/v2/messages";
    let local_pow = client.get_local_pow().await;
    let timeout = if local_pow {
        client.get_timeout()
    } else {
        client.get_remote_pow_timeout()
    };

    #[cfg(not(feature = "pow-fallback"))]
    let resp: SuccessBody<SubmitMessageResponse> = client
        .node_manager
        .post_request_bytes(path, timeout, &message.pack_to_vec(), local_pow)
        .await?;

    #[cfg(feature = "pow-fallback")]
    // fallback to local PoW if remote PoW fails
    let resp: SuccessBody<SubmitMessageResponse> = match client
        .node_manager
        .post_request_bytes(path, timeout, &message.pack_to_vec(), local_pow)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            if let Error::NodeError(e) = e {
                // hornet and bee return different error messages
                if e == *"No available nodes with remote PoW"
                    || e.contains("proof of work is not enabled")
                    || e.contains("`PoW` not enabled")
                {
                    let mut client_network_info = client.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                    // switch to local PoW
                    client_network_info.local_pow = true;
                    drop(client_network_info);
                    #[cfg(not(feature = "wasm"))]
                    let msg_res = crate::api::finish_pow(client: &Client, message.payload().clone()).await;
                    #[cfg(feature = "wasm")]
                    let msg_res = {
                        let min_pow_score = client.get_min_pow_score().await?;
                        let network_id = client.get_network_id().await?;
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
                                client.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                            client_network_info.local_pow = false;
                            msg
                        }
                        Err(e) => {
                            // reset local PoW state
                            let mut client_network_info =
                                client.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                            client_network_info.local_pow = false;
                            return Err(e);
                        }
                    };
                    client
                        .node_manager
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

    let mut message_id_bytes = [0u8; 32];
    hex::decode_to_slice(resp.data.message_id, &mut message_id_bytes)?;
    Ok(MessageId::from(message_id_bytes))
}

/// Returns the MessageId of the submitted message.
/// POST JSON to /api/v2/messages endpoint
pub async fn post_message_json(client: &Client, message: &Message) -> Result<MessageId> {
    let path = "api/v2/messages";
    let local_pow = client.get_local_pow().await;
    let timeout = if local_pow {
        client.get_timeout()
    } else {
        client.get_remote_pow_timeout()
    };
    let message_dto = MessageDto::from(message);

    // fallback to local PoW if remote PoW fails
    let resp: SuccessBody<SubmitMessageResponse> = match client
        .node_manager
        .post_request_json(path, timeout, serde_json::to_value(message_dto)?, local_pow)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            if let Error::NodeError(e) = e {
                // hornet and bee return different error messages
                if e == *"No available nodes with remote PoW"
                    || e.contains("proof of work is not enabled")
                    || e.contains("`PoW` not enabled")
                {
                    let mut client_network_info = client.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                    // switch to local PoW
                    client_network_info.local_pow = true;
                    drop(client_network_info);
                    #[cfg(not(feature = "wasm"))]
                    let msg_res = crate::api::finish_pow(client, message.payload().cloned()).await;
                    #[cfg(feature = "wasm")]
                    let msg_res = {
                        let min_pow_score = client.get_min_pow_score().await?;
                        let network_id = client.get_network_id().await?;
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
                                client.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                            client_network_info.local_pow = false;
                            msg
                        }
                        Err(e) => {
                            // reset local PoW state
                            let mut client_network_info =
                                client.network_info.write().map_err(|_| crate::Error::PoisonError)?;
                            client_network_info.local_pow = false;
                            return Err(e);
                        }
                    };
                    let message_dto = MessageDto::from(&message_with_local_pow);

                    client
                        .node_manager
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

/// Returns the message that was included in the ledger for a given TransactionId
/// GET /api/v2/transactions/{transactionId}/included-message
pub async fn get_included_message(client: &Client, transaction_id: &TransactionId) -> Result<Message> {
    let path = &format!("api/v2/transactions/{}/included-message", transaction_id);

    let resp: SuccessBody<MessageResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;
    Ok(Message::try_from(&resp.data.0)?)
}

/// Get the milestone by the given milestone index.
/// GET /api/v2/milestones/{index} endpoint
pub async fn get_milestone(client: &Client, index: u32) -> Result<MilestoneResponse> {
    let path = &format!("api/v2/milestones/{}", index);

    let resp: SuccessBody<MilestoneResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    // converted to an object with a MessageId instead of a String
    // let milestone = resp.data;
    // let mut message_id = [0u8; 32];
    // hex::decode_to_slice(milestone.message_id, &mut message_id)?;
    // Ok(MilestoneResponse {
    //     index: milestone.milestone_index,
    //     message_id: MessageId::new(message_id),
    //     timestamp: milestone.timestamp,
    // })
    Ok(resp.data)
}

/// Gets all UTXO changes of a milestone by its milestone index
/// GET /api/v2/milestones/{index}/utxo-changes endpoint
pub async fn get_milestone_utxo_changes(client: &Client, index: u32) -> Result<UtxoChangesResponse> {
    let path = &format!("api/v2/milestones/{}/utxo-changes", index);

    let resp: SuccessBody<UtxoChangesResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    Ok(resp.data)
}

/// Find an output by its OutputId (TransactionId + output_index).
/// GET /api/v2/outputs/{outputId} endpoint
pub async fn get_output(client: &Client, output_id: &OutputId) -> Result<OutputResponse> {
    let path = &format!("api/v2/outputs/{}", output_id);

    let resp: SuccessBody<OutputResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    Ok(resp.data)
}

/// Get the current treasury output.
/// GET /api/v2/treasury endpoint
pub async fn get_treasury(client: &Client) -> Result<TreasuryResponse> {
    let path = "api/v2/treasury";

    let resp: SuccessBody<TreasuryResponse> = client.node_manager.get_request(path, None, DEFAULT_API_TIMEOUT).await?;

    Ok(resp.data)
}

/// Get all stored receipts.
/// GET /api/v2/receipts endpoint
pub async fn get_receipts(client: &Client) -> Result<Vec<ReceiptDto>> {
    let path = &"api/v2/receipts";

    let resp: SuccessBody<ReceiptsResponse> = client.node_manager.get_request(path, None, DEFAULT_API_TIMEOUT).await?;

    Ok(resp.data.receipts)
}

/// Get the receipts by the given milestone index.
/// GET /api/v2/receipts/{migratedAt} endpoint
pub async fn get_receipts_migrated_at(client: &Client, milestone_index: u32) -> Result<Vec<ReceiptDto>> {
    let path = &format!("api/v2/receipts/{}", milestone_index);

    let resp: SuccessBody<ReceiptsResponse> = client.node_manager.get_request(path, None, DEFAULT_API_TIMEOUT).await?;

    Ok(resp.data.receipts)
}

// // RoutePeer is the route for getting peers by their peerID.
// // GET returns the peer
// // DELETE deletes the peer.
// RoutePeer = "/peers/:" + restapipkg.ParameterPeerID

// // RoutePeers is the route for getting all peers of the node.
// // GET returns a list of all peers.
// // POST adds a new peer.
// RoutePeers = "/peers"

/// GET /api/v2/peers endpoint
pub async fn get_peers(client: &Client) -> Result<Vec<PeerDto>> {
    let path = "api/v2/peers";

    let resp: SuccessBody<PeersResponse> = client
        .node_manager
        .get_request(path, None, client.get_timeout())
        .await?;

    Ok(resp.data.0)
}

// // RouteControlDatabasePrune is the control route to manually prune the database.
// // POST prunes the database.
// RouteControlDatabasePrune = "/control/database/prune"

// // RouteControlSnapshotsCreate is the control route to manually create a snapshot files.
// // POST creates a snapshot (full, delta or both).
// RouteControlSnapshotsCreate = "/control/snapshots/create"
