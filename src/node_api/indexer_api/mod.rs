// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node indexer API
use crate::{
    node_api::indexer_api::{
        query_parameters::{QueryParameter, QueryParameters},
        responses::ExtendedOutputsResponse,
    },
    Api, Client, Error, Result,
};

use bee_message::{input::UtxoInput, output::OutputId, payload::transaction::TransactionId};
use bee_rest_api::types::{dtos::OutputDto, responses::BalanceAddressResponse};

use std::convert::TryInto;

const OUTPUT_ID_LENGTH: usize = 68;
const TRANSACTION_ID_LENGTH: usize = 64;

pub mod query_parameters;
pub mod responses;

// https://github.com/gohornet/hornet/blob/stardust-utxo/plugins/indexer/routes.go

// // RouteOutputs is the route for getting outputs filtered by the given parameters.
// 	// GET with query parameter returns all outputIDs that fit these filter criteria (query parameters: "address",
// "requiresDustReturn", "sender", "tag"). 	// Returns an empty list if no results are found.
// 	RouteOutputs = "/outputs"

/// api/plugins/indexer/outputs
pub async fn outputs(client: &Client, mut query_parameters: QueryParameters) -> Result<Vec<OutputId>> {
    // do we need to validate the query parameters?
    let path = "api/plugins/indexer/outputs";

    let mut all_output_ids: Vec<OutputId> = Vec::new();
    while let Some(offset) = {
        let outputs_response: ExtendedOutputsResponse = client
            .node_manager
            .get_request(
                path,
                query_parameters.into_query_sting().as_deref(),
                client.get_timeout(Api::GetOutput),
            )
            .await?;
        // convert string response to output ids
        let output_ids = outputs_response
            .data
            .iter()
            .map(|s| {
                if s.len() == OUTPUT_ID_LENGTH {
                    let mut transaction_id = [0u8; 32];
                    hex::decode_to_slice(&s[..TRANSACTION_ID_LENGTH], &mut transaction_id)?;
                    let index = u16::from_le_bytes(
                        hex::decode(&s[TRANSACTION_ID_LENGTH..]).map_err(|_| Error::InvalidParameter("index"))?[..]
                            .try_into()
                            .map_err(|_| Error::InvalidParameter("index"))?,
                    );
                    Ok(OutputId::new(TransactionId::new(transaction_id), index)?)
                } else {
                    Err(Error::OutputError("Invalid output length from API response"))
                }
            })
            .collect::<Result<Vec<OutputId>>>()?;
        all_output_ids.extend(output_ids.into_iter());
        outputs_response.offset
    } {
        query_parameters.replace(QueryParameter::Offset(offset));
    }

    Ok(all_output_ids)
}

// 	// RouteAliases is the route for getting aliases filtered by the given parameters.
// 	// GET with query parameter  returns all outputIDs that fit these filter criteria (query parameters:
// "stateController", "governor", "issuer", "sender"). 	// Returns an empty list if no results are found.
// 	RouteAliases = "/aliases"

// 	// RouteAliasByID is the route for getting aliases by their aliasID.
// 	// GET returns the outputIDs or 404 if no record is found.
// 	RouteAliasByID = "/aliases/:" + restapi.ParameterAliasID

// 	// RouteNFT is the route for getting NFT filtered by the given parameters.
// 	// GET with query parameter returns all outputIDs that fit these filter criteria (query parameters: "address",
// "requiresDustReturn", "issuer", "sender", "tag"). 	// Returns an empty list if no results are found.
// 	RouteNFT = "/nft"

// 	// RouteNFTByID is the route for getting NFT by their nftID.
// 	// GET returns the outputIDs or 404 if no record is found.
// 	RouteNFTByID = "/nft/:" + restapi.ParameterNFTID

// 	// RouteFoundries is the route for getting foundries filtered by the given parameters.
// 	// GET with query parameter returns all outputIDs that fit these filter criteria (query parameters: "address").
// 	// Returns an empty list if no results are found.
// 	RouteFoundries = "/foundries"

// 	// RouteFoundryByID is the route for getting foundries by their foundryID.
// 	// GET returns the outputIDs or 404 if no record is found.
// 	RouteFoundryByID = "/foundries/:" + restapi.ParameterFoundryID
