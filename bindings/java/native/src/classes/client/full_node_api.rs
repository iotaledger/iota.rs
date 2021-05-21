// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};

use std::{
    convert::{From, Into},
    str::FromStr,
};

use iota_client::{
    bee_message::{input::UtxoInput as RustUtxoInput, MessageId},
    client::Client as ClientRust,
};

use crate::{
    address::*, 
    bee_types::*, 
    client_builder::ClientBuilder, 
    message::{
        MessageWrap, ClientMessageBuilder,
    },
    balance::GetBalanceBuilderApi,
};

impl From<ClientRust> for Client {
    fn from(client: ClientRust) -> Self {
        Self(client)
    }
}

pub struct Client(ClientRust);

/// Full node API
impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn borrow<'a>(&'a self) -> &'a ClientRust {
        &self.0
    }

    pub fn get_health(&self) -> Result<bool> {
        Ok(crate::block_on(async { self.0.get_health().await })?)
    }

    pub fn get_node_health(&self, node: &str) -> Result<bool> {
        Ok(crate::block_on(async {
            iota_client::Client::get_node_health(node).await
        })?)
    }

    pub fn get_info(&self) -> Result<NodeInfoWrapper> {
        Ok(crate::block_on(async { self.0.get_info().await })?.into())
    }

    pub fn get_tips(&self) -> Result<Vec<String>> {
        let tips = crate::block_on(async { self.0.get_tips().await })?;
        Ok(tips.into_iter().map(|p| p.to_string()).collect())
    }

    pub fn get_peers(&self) -> Result<Vec<PeerDto>> {
        Ok(crate::block_on(async { self.0.get_peers().await })?
            .into_iter()
            .map(PeerDto::from)
            .collect())
    }

    // fn post_message(&self, msg: Message) -> Result<String> {
    // let rt = tokio::runtime::Runtime::new()?;
    // let mut msg_builder = RustMessageBuilder::<RustClientMiner>::new()
    // .with_network_id(msg.network_id)
    // .with_parents(Parents::new(
    // msg.parents
    // .iter()
    // .map(|m| m.parse::<RustMessageId>().expect("Invalid message id"))
    // .collect::<Vec<RustMessageId>>(),
    // )?)
    // .with_nonce_provider(rt.block_on(self.0.get_pow_provider()), 4000f64, None);
    // if let Some(payload) = msg.payload {
    // msg_builder = msg_builder.with_payload(payload.try_into()?);
    // }
    // let msg = msg_builder.finish()?;
    // Ok(crate::block_on(async { self.0.post_message(&msg).await })?.to_string())
    // }

    pub fn get_output(&self, output_id: String) -> Result<OutputResponse> {
        Ok(crate::block_on(async { self.0.get_output(&RustUtxoInput::from_str(&output_id)?).await })?.into())
    }

    /// GET /api/v1/addresses/{address} endpoint
    pub fn get_address(&self) -> GetAddressBuilderNode {
        GetAddressBuilderNode::new(self)
    }

    pub fn get_address_balance(&self, address: &str) -> Result<BalanceAddressResponse> {
        let mut result: BalanceAddressResponse =
            crate::block_on(async { self.0.get_address().balance(&String::from(address)).await })?.into();
        result.address = crate::block_on(async { self.0.hex_to_bech32(&result.address, None).await })?;
        Ok(result)
    }

    pub fn get_addresses_balances(&self, addresses: Vec<String>) -> Result<Vec<BalanceAddressResponse>> {
        let result: Vec<BalanceAddressResponse> =
            crate::block_on(async { self.0.get_address_balances(&addresses).await })?
                .into_iter()
                .map(|b| {
                    let result: BalanceAddressResponse = b.into();
                    // TODO
                    // result.address = self
                    // .block
                    // .block_on(async { self.0.hex_to_bech32(&result.address, None).await })?;
                    result
                })
                .collect();

        Ok(result)
    }

    pub fn find_outputs(
        &self,
        output_ids: Option<Vec<String>>,
        addresses: Option<Vec<String>>,
    ) -> Result<Vec<OutputResponse>> {
        let output_ids: Vec<RustUtxoInput> = output_ids
            .unwrap_or_default()
            .into_iter()
            .map(|input| RustUtxoInput::from_str(&input).unwrap_or_else(|_| panic!("invalid input: {}", input)))
            .collect();
        let output_metadata_vec = crate::block_on(async {
            self.0
                .find_outputs(&output_ids[..], &addresses.unwrap_or_default()[..])
                .await
        })?;
        Ok(output_metadata_vec
            .into_iter()
            .map(|metadata| metadata.into())
            .collect())
    }

    pub fn get_milestone(&self, index: u32) -> Result<MilestoneResponse> {
        Ok(crate::block_on(async { self.0.get_milestone(index).await })?.into())
    }

    pub fn get_milestone_utxo_changes(&self, index: u32) -> Result<MilestoneUtxoChangesResponse> {
        Ok(crate::block_on(async { self.0.get_milestone_utxo_changes(index).await })?.into())
    }

    pub fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let receipts: Vec<ReceiptDto> = crate::block_on(async { self.0.get_receipts().await })?
            .into_iter()
            .map(|r| r.into())
            .collect();
        Ok(receipts)
    }

    pub fn get_receipts_migrated_at(&self, index: u32) -> Result<Vec<ReceiptDto>> {
        let receipts: Vec<ReceiptDto> = crate::block_on(async { self.0.get_receipts_migrated_at(index).await })?
            .into_iter()
            .map(|r| r.into())
            .collect();
        Ok(receipts)
    }

    // fn get_treasury(&self) -> Result<TreasuryResponse> {
    // Ok(crate::block_on(async { self.0.get_treasury().await })?.into())
    // }

    // fn get_included_message(&self, input: String) -> Result<Message> {
    // let transaction_id = RustTransactionId::from_str(&input[..])?;
    // crate::block_on(async { self.0.get_included_message(&transaction_id).await })?.try_into()
    // }

    /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub fn reattach(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = crate::block_on(async { self.0.reattach(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Reattach a message without checking if it should be reattached
    pub fn reattach_unchecked(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = crate::block_on(async { self.0.reattach_unchecked(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
    /// method should error out and should not allow unnecessary promotions.
    pub fn promote(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = crate::block_on(async { self.0.promote(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Promote a message without checking if it should be promoted
    pub fn promote_unchecked(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = crate::block_on(async { self.0.promote_unchecked(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    // HIGH LEVEL API

    /// Return the balance for a provided seed and its wallet chain account index.
    /// Addresses with balance must be consecutive, so this method will return once it encounters a zero
    /// balance address.
    pub fn get_balance(&self, seed: &str) -> GetBalanceBuilderApi {
        GetBalanceBuilderApi::new(self, seed)
    }

    pub fn message(&self) -> ClientMessageBuilder {
        ClientMessageBuilder::new(self)
    }

    // UTIL BELOW

    pub fn bech32_to_hex(bech32: &str) -> Result<String> {
        let res = iota_client::Client::bech32_to_hex(bech32);
        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn hex_to_bech32(&self, hex: &str, bech32_hrp: Option<&str>) -> Result<String> {
        let res = crate::block_on(async { self.0.hex_to_bech32(hex, bech32_hrp).await }).into();
        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Returns a valid Address parsed from a String.
    pub fn parse_bech32_address(address: &str) -> Result<Address> {
        let res = iota_client::Client::parse_bech32_address(address);
        match res {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Checks if a String address is valid.
    pub fn is_address_valid(address: &str) -> bool {
        iota_client::Client::is_address_valid(address)
    }
}
