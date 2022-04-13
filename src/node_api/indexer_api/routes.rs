// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer routes
use bee_message::output::{AliasId, FoundryId, NftId, OutputId};

use crate::{
    node_api::indexer_api::{get_output_ids_with_pagination, query_parameters::QueryParameter},
    Client, Result,
};

// hornet: https://github.com/gohornet/hornet/blob/develop/plugins/indexer/routes.go

/// Get outputs filtered by the given parameters.
/// GET with query parameter returns all outputIDs that fit these filter criteria.
/// Query parameters: "address", "hasStorageDepositReturnCondition", "storageReturnAddress", "hasExpirationCondition",
///                 "expiresBefore", "expiresAfter", "expiresBeforeMilestone", "expiresAfterMilestone",
///                 "hasTimelockCondition", "timelockedBefore", "timelockedAfter", "timelockedBeforeMilestone",
///                 "timelockedAfterMilestone", "sender", "tag", "createdBefore", "createdAfter"
/// Returns an empty Vec if no results are found.
/// api/plugins/indexer/v1/outputs
pub async fn output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/outputs";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

/// Get aliases filtered by the given parameters.
/// GET with query parameter returns all outputIDs that fit these filter criteria.
/// Query parameters: "stateController", "governor", "issuer", "sender", "createdBefore", "createdAfter"
/// Returns an empty list if no results are found.
/// api/plugins/indexer/v1/aliases
pub async fn aliases_output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/aliases";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

/// Get aliases by their aliasID.
/// api/plugins/indexer/v1/aliases/:{AliasId}
pub async fn alias_output_id(client: &Client, alias_id: AliasId) -> Result<OutputId> {
    let route = format!("api/plugins/indexer/v1/aliases/{alias_id}");

    Ok(*(get_output_ids_with_pagination(client, &route, Vec::new())
        .await?
        .first()
        .ok_or_else(|| crate::Error::NodeError("No output id for alias".to_string()))?))
}

/// Get NFT filtered by the given parameters.
/// Query parameters: "address", "hasStorageDepositReturnCondition", "storageReturnAddress", "hasExpirationCondition",
///                 "expiresBefore", "expiresAfter", "expiresBeforeMilestone", "expiresAfterMilestone",
///                 "hasTimelockCondition", "timelockedBefore", "timelockedAfter", "timelockedBeforeMilestone",
///                 "timelockedAfterMilestone", "issuer", "sender", "tag", "createdBefore", "createdAfter"
/// Returns an empty list if no results are found.
/// api/plugins/indexer/v1/nfts
pub async fn nfts_output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/nfts";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

/// Get NFT by their nftID.
/// api/plugins/indexer/v1/nfts/:{NftId}
pub async fn nft_output_id(client: &Client, nft_id: NftId) -> Result<OutputId> {
    let route = format!("api/plugins/indexer/v1/nfts/{nft_id}");

    Ok(*(get_output_ids_with_pagination(client, &route, Vec::new())
        .await?
        .first()
        .ok_or_else(|| crate::Error::NodeError("No output id for nft".to_string()))?))
}

/// Get foundries filtered by the given parameters.
/// GET with query parameter returns all outputIDs that fit these filter criteria.
/// Query parameters: "address", "createdBefore", "createdAfter"
/// Returns an empty list if no results are found.
/// api/plugins/indexer/v1/foundries
pub async fn foundries_output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/foundries";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

/// Get foundries by their foundryID.
/// api/plugins/indexer/v1/foundries/:{FoundryID}
pub async fn foundry_output_id(client: &Client, foundry_id: FoundryId) -> Result<OutputId> {
    let route = format!("api/plugins/indexer/v1/foundries/{foundry_id}");

    Ok(*(get_output_ids_with_pagination(client, &route, Vec::new())
        .await?
        .first()
        .ok_or_else(|| crate::Error::NodeError("No output id for foundry".to_string()))?))
}
