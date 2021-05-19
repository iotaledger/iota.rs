// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};

use std::{
    convert::{From, Into},
    str::FromStr,
};

use iota_client::{
    bee_message::{
        input::UtxoInput as RustUTXOInput,
        MessageId,
    },
    client::Client as ClientRust,
};

use crate::{address::*, message::MessageWrap, bee_types::*, client_builder::ClientBuilder};

use tokio::runtime::Runtime;

impl From<ClientRust> for Client {
    fn from(client: ClientRust) -> Self {
        Self {
            client: client,
            block: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }
}

pub struct Client {
    block: Runtime,
    client: ClientRust,
}

/// Full node API
impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn borrow<'a>(&'a self) -> &'a ClientRust {
        &self.client
    }

    pub fn get_health(&self) -> Result<bool> {
        Ok(self.block.block_on(async { self.client.get_health().await })?)
    }

    pub fn get_node_health(&self, node: &str) -> Result<bool> {
        Ok(self
            .block
            .block_on(async { iota_client::Client::get_node_health(node).await })?)
    }

    pub fn get_info(&self) -> Result<NodeInfoWrapper> {
        Ok(self.block.block_on(async { self.client.get_info().await })?.into())
    }

    pub fn get_tips(&self) -> Result<Vec<String>> {
        let tips = self.block.block_on(async { self.client.get_tips().await })?;
        Ok(tips.into_iter().map(|p| p.to_string()).collect())
    }

    pub fn get_peers(&self) -> Result<Vec<PeerDto>> {
        Ok(self
            .block
            .block_on(async { self.client.get_peers().await })?
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
    // .with_nonce_provider(rt.block_on(self.client.get_pow_provider()), 4000f64, None);
    // if let Some(payload) = msg.payload {
    // msg_builder = msg_builder.with_payload(payload.try_into()?);
    // }
    // let msg = msg_builder.finish()?;
    // Ok(crate::block_on(async { self.client.post_message(&msg).await })?.to_string())
    // }

    pub fn get_output(&self, output_id: String) -> Result<OutputResponse> {
        Ok(self
            .block
            .block_on(async { self.client.get_output(&RustUTXOInput::from_str(&output_id)?).await })?
            .into())
    }

    pub fn get_address_balance(&self, address: &str) -> Result<BalanceAddressResponse> {
        let mut result: BalanceAddressResponse = self
            .block
            .block_on(async { self.client.get_address().balance(&String::from(address)).await })?
            .into();
        result.address = self
            .block
            .block_on(async { self.client.hex_to_bech32(&result.address, None).await })?;
        Ok(result)
    }

    pub fn get_addresses_balances(&self, addresses: Vec<String>) -> Result<Vec<BalanceAddressResponse>> {
        let mut result: Vec<BalanceAddressResponse> = self
            .block
            .block_on(async { self.client.get_address_balances(&addresses).await })?
            .into_iter()
            .map(|b| {
                let mut result: BalanceAddressResponse = b.into();
                // TODO
                // result.address = self
                // .block
                // .block_on(async { self.client.hex_to_bech32(&result.address, None).await })?;
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
        let output_ids: Vec<RustUTXOInput> = output_ids
            .unwrap_or_default()
            .into_iter()
            .map(|input| RustUTXOInput::from_str(&input).unwrap_or_else(|_| panic!("invalid input: {}", input)))
            .collect();
        let output_metadata_vec = crate::block_on(async {
            self.client
                .find_outputs(&output_ids[..], &addresses.unwrap_or_default()[..])
                .await
        })?;
        Ok(output_metadata_vec
            .into_iter()
            .map(|metadata| metadata.into())
            .collect())
    }

    pub fn get_milestone(&self, index: u32) -> Result<MilestoneResponse> {
        Ok(self
            .block
            .block_on(async { self.client.get_milestone(index).await })?
            .into())
    }

    pub fn get_milestone_utxo_changes(&self, index: u32) -> Result<MilestoneUtxoChangesResponse> {
        Ok(self
            .block
            .block_on(async { self.client.get_milestone_utxo_changes(index).await })?
            .into())
    }

    pub fn get_receipts(&self) -> Result<Vec<ReceiptDto>> {
        let receipts: Vec<ReceiptDto> = self
            .block
            .block_on(async { self.client.get_receipts().await })?
            .into_iter()
            .map(|r| r.into())
            .collect();
        Ok(receipts)
    }

    pub fn get_receipts_migrated_at(&self, index: u32) -> Result<Vec<ReceiptDto>> {
        let receipts: Vec<ReceiptDto> = self
            .block
            .block_on(async { self.client.get_receipts_migrated_at(index).await })?
            .into_iter()
            .map(|r| r.into())
            .collect();
        Ok(receipts)
    }

    // fn get_treasury(&self) -> Result<TreasuryResponse> {
    // Ok(crate::block_on(async { self.client.get_treasury().await })?.into())
    // }

    // fn get_included_message(&self, input: String) -> Result<Message> {
    // let transaction_id = RustTransactionId::from_str(&input[..])?;
    // crate::block_on(async { self.client.get_included_message(&transaction_id).await })?.try_into()
    // }

    /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub fn reattach(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = self
            .block
            .block_on(async { self.client.reattach(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Reattach a message without checking if it should be reattached
    pub fn reattach_unchecked(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = self
            .block
            .block_on(async { self.client.reattach_unchecked(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
    /// method should error out and should not allow unnecessary promotions.
    pub fn promote(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = self
            .block
            .block_on(async { self.client.promote(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Promote a message without checking if it should be promoted
    pub fn promote_unchecked(&self, message_id: MessageId) -> Result<MessageWrap> {
        let res = self
            .block
            .block_on(async { self.client.promote_unchecked(&message_id).await });

        match res {
            Ok(w) => Ok(MessageWrap::new(w.0, w.1.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
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
        let res = self
            .block
            .block_on(async { self.client.hex_to_bech32(hex, bech32_hrp).await })
            .into();
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
