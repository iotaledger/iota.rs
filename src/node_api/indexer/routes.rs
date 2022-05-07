// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer routes
use bee_message::output::{AliasId, FoundryId, NftId, OutputId};

use crate::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

// hornet: https://github.com/gohornet/hornet/blob/develop/plugins/indexer/routes.go

impl Client {
    /// Get outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "address", "hasStorageDepositReturnCondition", "storageReturnAddress",
    /// "hasExpirationCondition",                 "expiresBefore", "expiresAfter", "expiresBeforeMilestone",
    /// "expiresAfterMilestone",                 "hasTimelockCondition", "timelockedBefore", "timelockedAfter",
    /// "timelockedBeforeMilestone",                 "timelockedAfterMilestone", "sender", "tag", "createdBefore",
    /// "createdAfter" Returns an empty Vec if no results are found.
    /// api/plugins/indexer/v1/outputs/basic
    pub async fn output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
        let route = "api/plugins/indexer/v1/outputs/basic";

        self.get_output_ids_with_pagination(route, query_parameters).await
    }

    /// Get aliases filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "stateController", "governor", "issuer", "sender", "createdBefore", "createdAfter"
    /// Returns an empty list if no results are found.
    /// api/plugins/indexer/v1/outputs/alias
    pub async fn aliases_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
        let route = "api/plugins/indexer/v1/outputs/alias";

        self.get_output_ids_with_pagination(route, query_parameters).await
    }

    /// Get aliases by their aliasID.
    /// api/plugins/indexer/v1/outputs/alias/:{AliasId}
    pub async fn alias_output_id(&self, alias_id: AliasId) -> Result<OutputId> {
        let route = format!("api/plugins/indexer/v1/outputs/alias/{alias_id}");

        Ok(*(self
            .get_output_ids_with_pagination(&route, Vec::new())
            .await?
            .first()
            .ok_or_else(|| crate::Error::NodeError("No output id for alias".to_string()))?))
    }

    /// Get NFT filtered by the given parameters.
    /// Query parameters: "address", "hasStorageDepositReturnCondition", "storageReturnAddress",
    /// "hasExpirationCondition",                 "expiresBefore", "expiresAfter", "expiresBeforeMilestone",
    /// "expiresAfterMilestone",                 "hasTimelockCondition", "timelockedBefore", "timelockedAfter",
    /// "timelockedBeforeMilestone",                 "timelockedAfterMilestone", "issuer", "sender", "tag",
    /// "createdBefore", "createdAfter" Returns an empty list if no results are found.
    /// api/plugins/indexer/v1/outputs/nft
    pub async fn nfts_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
        let route = "api/plugins/indexer/v1/outputs/nft";

        self.get_output_ids_with_pagination(route, query_parameters).await
    }

    /// Get NFT by their nftID.
    /// api/plugins/indexer/v1/outputs/nft/:{NftId}
    pub async fn nft_output_id(&self, nft_id: NftId) -> Result<OutputId> {
        let route = format!("api/plugins/indexer/v1/outputs/nft/{nft_id}");

        Ok(*(self
            .get_output_ids_with_pagination(&route, Vec::new())
            .await?
            .first()
            .ok_or_else(|| crate::Error::NodeError("No output id for nft".to_string()))?))
    }

    /// Get foundries filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "address", "createdBefore", "createdAfter"
    /// Returns an empty list if no results are found.
    /// api/plugins/indexer/v1/outputs/foundry
    pub async fn foundries_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
        let route = "api/plugins/indexer/v1/outputs/foundry";

        self.get_output_ids_with_pagination(route, query_parameters).await
    }

    /// Get foundries by their foundryID.
    /// api/plugins/indexer/v1/outputs/foundry/:{FoundryID}
    pub async fn foundry_output_id(&self, foundry_id: FoundryId) -> Result<OutputId> {
        let route = format!("api/plugins/indexer/v1/outputs/foundry/{foundry_id}");

        Ok(*(self
            .get_output_ids_with_pagination(&route, Vec::new())
            .await?
            .first()
            .ok_or_else(|| crate::Error::NodeError("No output id for foundry".to_string()))?))
    }
}
