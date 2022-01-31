// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer routes
use crate::{
    node_api::indexer_api::{get_output_ids_with_pagination, query_parameters::QueryParameter},
    Client, Result,
};

use bee_message::output::{AliasId, FoundryId, NftId, OutputId};

// hornet: https://github.com/gohornet/hornet/blob/stardust-utxo/plugins/indexer/v1/routes.go

// // RouteOutputs is the route for getting outputs filtered by the given parameters.
// 	// GET with query parameter returns all outputIDs that fit these filter criteria (query parameters: "address",
// "requiresDustReturn", "sender", "tag"). 	// Returns an empty list if no results are found.
// 	RouteOutputs = "/outputs"

/// api/plugins/indexer/v1/outputs
pub async fn output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/outputs";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

// 	// RouteAliases is the route for getting aliases filtered by the given parameters.
// 	// GET with query parameter  returns all outputIDs that fit these filter criteria (query parameters:
// "stateController", "governor", "issuer", "sender"). 	// Returns an empty list if no results are found.
// 	RouteAliases = "/aliases"

/// api/plugins/indexer/v1/aliases
pub async fn aliases_output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/aliases";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

// 	// RouteAliasByID is the route for getting aliases by their aliasID.
// 	// GET returns the outputIDs or 404 if no record is found.
// 	RouteAliasByID = "/aliases/:" + restapi.ParameterAliasID

/// api/plugins/indexer/v1/aliases/:{AliasId}
pub async fn alias_output_ids(client: &Client, alias_id: AliasId) -> Result<Vec<OutputId>> {
    let route = format!("api/plugins/indexer/v1/aliases/{alias_id}");

    get_output_ids_with_pagination(client, &route, Vec::new()).await
}

// 	// RouteNFT is the route for getting NFT filtered by the given parameters.
// 	// GET with query parameter returns all outputIDs that fit these filter criteria (query parameters: "address",
// "requiresDustReturn", "issuer", "sender", "tag"). 	// Returns an empty list if no results are found.
// 	RouteNFT = "/nft"

/// api/plugins/indexer/v1/nft
pub async fn nfts_output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/nft";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

// 	// RouteNFTByID is the route for getting NFT by their nftID.
// 	// GET returns the outputIDs or 404 if no record is found.
// 	RouteNFTByID = "/nft/:" + restapi.ParameterNFTID

/// api/plugins/indexer/v1/nft/:{NftId}
pub async fn nft_output_ids(client: &Client, nft_id: NftId) -> Result<Vec<OutputId>> {
    let route = format!("api/plugins/indexer/v1/nft/{nft_id}");

    get_output_ids_with_pagination(client, &route, Vec::new()).await
}

// 	// RouteFoundries is the route for getting foundries filtered by the given parameters.
// 	// GET with query parameter returns all outputIDs that fit these filter criteria (query parameters: "address").
// 	// Returns an empty list if no results are found.
// 	RouteFoundries = "/foundries"

/// api/plugins/indexer/v1/foundries
pub async fn foundries_output_ids(client: &Client, query_parameters: Vec<QueryParameter>) -> Result<Vec<OutputId>> {
    let route = "api/plugins/indexer/v1/foundries";

    get_output_ids_with_pagination(client, route, query_parameters).await
}

// 	// RouteFoundryByID is the route for getting foundries by their foundryID.
// 	// GET returns the outputIDs or 404 if no record is found.
// 	RouteFoundryByID = "/foundries/:" + restapi.ParameterFoundryID

/// api/plugins/indexer/v1/foundries/:{FoundryID}
pub async fn foundry_output_ids(client: &Client, foundry_id: FoundryId) -> Result<Vec<OutputId>> {
    let route = format!("api/plugins/indexer/v1/foundries/{foundry_id}");

    get_output_ids_with_pagination(client, &route, Vec::new()).await
}
