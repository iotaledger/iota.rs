// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::{
    error::Result, BalanceForAddressResponse, Client, InfoResponse, Message, MilestoneDto, MilestoneUTXOChanges,
    OutputResponse, PeerDto, UTXOInput,
};
use iota::{
    Bech32Address as RustBech32Address, ClientMiner as RustClientMiner, MessageBuilder as RustMessageBuilder,
    MessageId as RustMessageId, UTXOInput as RustUTXOInput,
};
use pyo3::prelude::*;

use std::{
    convert::{From, Into, TryInto},
    str::FromStr,
};

/// Full node API
#[pymethods]
impl Client {
    fn get_health(&self) -> Result<bool> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt.block_on(async { self.client.get_health().await })?)
    }
    fn get_info(&self) -> Result<InfoResponse> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt.block_on(async { self.client.get_info().await })?.into())
    }
    fn get_peers(&self) -> Result<Vec<PeerDto>> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt
            .block_on(async { self.client.get_peers().await })?
            .into_iter()
            .map(PeerDto::from)
            .collect())
    }
    fn get_tips(&self) -> Result<Vec<String>> {
        let rt = tokio::runtime::Runtime::new()?;
        let tips = rt.block_on(async { self.client.get_tips().await })?;
        Ok(tips.into_iter().map(|p| p.to_string()).collect())
    }
    fn post_message(&self, msg: Message) -> Result<String> {
        let mut msg_builder = RustMessageBuilder::<RustClientMiner>::new()
            .with_network_id(msg.network_id)
            .with_parents(
                msg.parents
                    .iter()
                    .map(|m| m.parse::<RustMessageId>().expect("Invalid message id"))
                    .collect::<Vec<RustMessageId>>(),
            )
            .with_nonce_provider(self.client.get_pow_provider(), 4000f64, None);
        if let Some(payload) = msg.payload {
            msg_builder = msg_builder.with_payload(payload.try_into()?);
        }
        let msg = msg_builder.finish()?;
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt.block_on(async { self.client.post_message(&msg).await })?.to_string())
    }
    fn get_output(&self, output_id: String) -> Result<OutputResponse> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt
            .block_on(async { self.client.get_output(&RustUTXOInput::from_str(&output_id)?).await })?
            .into())
    }
    fn get_address_balance(&self, address: &str) -> Result<BalanceForAddressResponse> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        Ok(rt
            .block_on(async {
                self.client
                    .get_address()
                    .balance(&RustBech32Address::from(address))
                    .await
            })?
            .into())
    }
    fn get_address_outputs(&self, address: &str) -> Result<Vec<UTXOInput>> {
        let rt = tokio::runtime::Runtime::new()?;
        let outputs = rt.block_on(async {
            self.client
                .get_address()
                .outputs(&RustBech32Address::from(address))
                .await
        })?;
        Ok((*outputs)
            .to_vec()
            .iter()
            .map(|output| UTXOInput {
                transaction_id: output.output_id().transaction_id().as_ref().to_vec(),
                index: output.output_id().index(),
            })
            .collect())
    }
    fn find_outputs(
        &self,
        output_ids: Option<Vec<String>>,
        addresses: Option<Vec<String>>,
    ) -> Result<Vec<OutputResponse>> {
        let output_ids: Vec<RustUTXOInput> = output_ids
            .unwrap_or_default()
            .iter()
            .map(|input| RustUTXOInput::from_str(input).unwrap_or_else(|_| panic!("invalid input: {}", input)))
            .collect();
        let addresses: Vec<RustBech32Address> = addresses
            .unwrap_or_default()
            .iter()
            .map(|address| RustBech32Address::from(&address[..]))
            .collect();
        let rt = tokio::runtime::Runtime::new()?;
        let output_metadata_vec =
            rt.block_on(async { self.client.find_outputs(&output_ids[..], &addresses[..]).await })?;
        Ok(output_metadata_vec
            .into_iter()
            .map(|metadata| metadata.into())
            .collect())
    }
    fn get_milestone(&self, index: u32) -> Result<MilestoneDto> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt.block_on(async { self.client.get_milestone(index).await })?.into())
    }
    fn get_milestone_utxo_changes(&self, index: u32) -> Result<MilestoneUTXOChanges> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt
            .block_on(async { self.client.get_milestone_utxo_changes(index).await })?
            .into())
    }
}
