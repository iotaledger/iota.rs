// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::{
    error::Result, AddressOutputsOptions, BalanceAddressResponse, Client, Message, MilestoneDto, MilestoneUTXOChanges,
    NodeInfoWrapper, OutputResponse, PeerDto, ReceiptDto, TreasuryResponse, UtxoInput,
};
use iota_client::{
    bee_message::prelude::{
        MessageBuilder as RustMessageBuilder, MessageId as RustMessageId, Parents, TransactionId as RustTransactionId,
        UtxoInput as RustUtxoInput,
    },
    ClientMiner as RustClientMiner,
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
        Ok(crate::block_on(async { self.client.get_health().await })?)
    }
    fn get_info(&self) -> Result<NodeInfoWrapper> {
        Ok(crate::block_on(async { self.client.get_info().await })?.into())
    }
    fn get_peers(&self) -> Result<Vec<PeerDto>> {
        Ok(crate::block_on(async { self.client.get_peers().await })?
            .into_iter()
            .map(PeerDto::from)
            .collect())
    }
    fn get_tips(&self) -> Result<Vec<String>> {
        let tips = crate::block_on(async { self.client.get_tips().await })?;
        Ok(tips.into_iter().map(|p| p.to_string()).collect())
    }
    fn post_message(&self, msg: Message) -> Result<String> {
        let rt = tokio::runtime::Runtime::new()?;
        let mut msg_builder = RustMessageBuilder::<RustClientMiner>::new()
            .with_network_id(msg.network_id)
            .with_parents(Parents::new(
                msg.parents
                    .iter()
                    .map(|m| m.parse::<RustMessageId>().expect("Invalid message id"))
                    .collect::<Vec<RustMessageId>>(),
            )?)
            .with_nonce_provider(rt.block_on(self.client.get_pow_provider()), 4000f64);
        if let Some(payload) = msg.payload {
            msg_builder = msg_builder.with_payload(payload.try_into()?);
        }
        let msg = msg_builder.finish()?;
        Ok(crate::block_on(async { self.client.post_message(&msg).await })?.to_string())
    }
    fn get_output(&self, output_id: String) -> Result<OutputResponse> {
        let mut result: OutputResponse =
            crate::block_on(async { self.client.get_output(&RustUtxoInput::from_str(&output_id)?).await })?.into();
        if let Some(ref mut output) = result.output.signature_locked_single {
            output.address.ed25519.address =
                crate::block_on(async { self.client.hex_to_bech32(&output.address.ed25519.address, None).await })?;
        } else if let Some(ref mut output) = result.output.signature_locked_dust_allowance {
            output.address.ed25519.address =
                crate::block_on(async { self.client.hex_to_bech32(&output.address.ed25519.address, None).await })?;
        }
        Ok(result)
    }
    fn get_address_balance(&self, address: &str) -> Result<BalanceAddressResponse> {
        let mut result: BalanceAddressResponse =
            crate::block_on(async { self.client.get_address().balance(&String::from(address)).await })?.into();
        result.address = crate::block_on(async { self.client.hex_to_bech32(&result.address, None).await })?;
        Ok(result)
    }
    fn get_address_outputs(&self, address: &str, options: Option<AddressOutputsOptions>) -> Result<Vec<UtxoInput>> {
        let outputs = crate::block_on(async {
            self.client
                .get_address()
                .outputs(&String::from(address), options.map(|o| o.into()).unwrap_or_default())
                .await
        })?;
        Ok((*outputs)
            .to_vec()
            .iter()
            .map(|output| UtxoInput {
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
        let output_ids: Vec<RustUtxoInput> = output_ids
            .unwrap_or_default()
            .iter()
            .map(|input| RustUtxoInput::from_str(input).unwrap_or_else(|_| panic!("invalid input: {input}")))
            .collect();
        let addresses: Vec<String> = addresses
            .unwrap_or_default()
            .iter()
            .map(|address| String::from(&address[..]))
            .collect();
        let output_metadata_vec =
            crate::block_on(async { self.client.find_outputs(&output_ids[..], &addresses[..]).await })?;
        Ok(output_metadata_vec
            .into_iter()
            .map(|metadata| {
                let mut response: OutputResponse = metadata.into();
                if let Some(ref mut output) = response.output.signature_locked_single {
                    output.address.ed25519.address = crate::block_on(async {
                        self.client.hex_to_bech32(&output.address.ed25519.address, None).await
                    })
                    .unwrap_or_else(|_| panic!("invalid bech32 address: {}", output.address.ed25519.address));
                } else if let Some(ref mut output) = response.output.signature_locked_dust_allowance {
                    output.address.ed25519.address = crate::block_on(async {
                        self.client.hex_to_bech32(&output.address.ed25519.address, None).await
                    })
                    .unwrap_or_else(|_| panic!("invalid bech32 address: {}", output.address.ed25519.address));
                }
                response
            })
            .collect())
    }
    fn get_milestone(&self, index: u32) -> Result<MilestoneDto> {
        Ok(crate::block_on(async { self.client.get_milestone(index).await })?.into())
    }
    fn get_milestone_utxo_changes(&self, index: u32) -> Result<MilestoneUTXOChanges> {
        Ok(crate::block_on(async { self.client.get_milestone_utxo_changes(index).await })?.into())
    }
    fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let receipts: Vec<ReceiptDto> = crate::block_on(async { self.client.get_receipts().await })?
            .into_iter()
            .map(|r| r.into())
            .collect();
        Ok(receipts)
    }
    fn get_receipts_migrated_at(&self, index: u32) -> Result<Vec<ReceiptDto>> {
        let receipts: Vec<ReceiptDto> = crate::block_on(async { self.client.get_receipts_migrated_at(index).await })?
            .into_iter()
            .map(|r| r.into())
            .collect();
        Ok(receipts)
    }
    fn get_treasury(&self) -> Result<TreasuryResponse> {
        Ok(crate::block_on(async { self.client.get_treasury().await })?.into())
    }
    fn get_included_message(&self, input: String) -> Result<Message> {
        let transaction_id = RustTransactionId::from_str(&input[..])?;
        crate::block_on(async { self.client.get_included_message(&transaction_id).await })?.try_into()
    }
}
