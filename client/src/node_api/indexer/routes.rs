// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer routes

use iota_types::{
    api::plugins::indexer::OutputIdsResponse,
    block::output::{AliasId, FoundryId, NftId, OutputId},
};

use crate::{
    node_api::indexer::{
        query_parameters::{
            verify_query_parameters_alias_outputs, verify_query_parameters_basic_outputs,
            verify_query_parameters_foundry_outputs, verify_query_parameters_nft_outputs, QueryParameter,
        },
        QueryParameters,
    },
    Client, Result,
};

// hornet: https://github.com/gohornet/hornet/blob/develop/plugins/indexer/routes.go

impl Client {
    /// Get basic outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "address", "hasStorageDepositReturn", "storageDepositReturnAddress",
    /// "hasExpiration", "expiresBefore", "expiresAfter", "hasTimelock", "timelockedBefore",
    /// "timelockedAfter", "sender", "tag", "createdBefore" and "createdAfter". Returns an empty Vec if no results
    /// are found. api/indexer/v1/outputs/basic
    pub async fn basic_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v1/outputs/basic";

        let query_parameters = verify_query_parameters_basic_outputs(query_parameters)?;

        self.get_output_ids(route, query_parameters, true, false).await
    }

    /// Get alias outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "stateController", "governor", "issuer", "sender", "createdBefore", "createdAfter"
    /// Returns an empty list if no results are found.
    /// api/indexer/v1/outputs/alias
    pub async fn alias_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v1/outputs/alias";

        let query_parameters = verify_query_parameters_alias_outputs(query_parameters)?;

        self.get_output_ids(route, query_parameters, true, false).await
    }

    /// Get alias output by its aliasID.
    /// api/indexer/v1/outputs/alias/:{AliasId}
    pub async fn alias_output_id(&self, alias_id: AliasId) -> Result<OutputId> {
        let route = format!("api/indexer/v1/outputs/alias/{alias_id}");

        Ok(*(self
            .get_output_ids(&route, QueryParameters::empty(), true, false)
            .await?
            .first()
            .ok_or_else(|| crate::Error::Node("no output id for alias".to_string()))?))
    }

    /// Get foundry outputs filtered by the given parameters.
    /// GET with query parameter returns all outputIDs that fit these filter criteria.
    /// Query parameters: "address", "createdBefore", "createdAfter"
    /// Returns an empty list if no results are found.
    /// api/indexer/v1/outputs/foundry
    pub async fn foundry_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v1/outputs/foundry";

        let query_parameters = verify_query_parameters_foundry_outputs(query_parameters)?;

        self.get_output_ids(route, query_parameters, true, false).await
    }

    /// Get foundry output by its foundryID.
    /// api/indexer/v1/outputs/foundry/:{FoundryID}
    pub async fn foundry_output_id(&self, foundry_id: FoundryId) -> Result<OutputId> {
        let route = format!("api/indexer/v1/outputs/foundry/{foundry_id}");

        Ok(*(self
            .get_output_ids(&route, QueryParameters::empty(), true, false)
            .await?
            .first()
            .ok_or_else(|| crate::Error::Node("no output id for foundry".to_string()))?))
    }

    /// Get NFT outputs filtered by the given parameters.
    /// Query parameters: "address", "hasStorageDepositReturn", "storageDepositReturnAddress",
    /// "hasExpiration", "expiresBefore", "expiresAfter", "hasTimelock", "timelockedBefore",
    /// "timelockedAfter", "issuer", "sender", "tag", "createdBefore", "createdAfter"
    /// Returns an empty list if no results are found.
    /// api/indexer/v1/outputs/nft
    pub async fn nft_output_ids(&self, query_parameters: Vec<QueryParameter>) -> Result<OutputIdsResponse> {
        let route = "api/indexer/v1/outputs/nft";

        let query_parameters = verify_query_parameters_nft_outputs(query_parameters)?;

        self.get_output_ids(route, query_parameters, true, false).await
    }

    /// Get NFT output by its nftID.
    /// api/indexer/v1/outputs/nft/:{NftId}
    pub async fn nft_output_id(&self, nft_id: NftId) -> Result<OutputId> {
        let route = format!("api/indexer/v1/outputs/nft/{nft_id}");

        Ok(*(self
            .get_output_ids(&route, QueryParameters::empty(), true, false)
            .await?
            .first()
            .ok_or_else(|| crate::Error::Node("no output id for nft".to_string()))?))
    }
}
