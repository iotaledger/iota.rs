// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
  address_getter::AddressGetter, balance_getter::BalanceGetter, client_builder::to_basic_auth, error::wasm_error,
  get_address::GetAddressBuilder, message_builder::MessageBuilder, message_getter::MessageGetter,
  unspent_address_getter::UnspentAddressGetter,
};
use iota_client::{
  bee_message::{
    input::UtxoInput, parents::Parents, payload::transaction::Essence, payload::transaction::TransactionId,
    payload::transaction::TransactionPayload, Message, MessageBuilder as RustMessageBuilder, MessageId,
  },
  bee_rest_api::types::dtos::{AddressDto, MessageDto as BeeMessageDto, OutputDto, PayloadDto, TransactionPayloadDto},
  common::packable::Packable,
  Client as RustClient, ClientMiner, Seed,
};
use js_sys::Promise;
use std::rc::Rc;
use std::{
  convert::{TryFrom, TryInto},
  str::FromStr,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

/// Struct for PostMessage
#[derive(Serialize, Deserialize)]
pub struct MessageDto {
  pub parents: Option<Vec<String>>,
  pub payload: PayloadDto,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Client {
  pub(crate) client: Rc<RustClient>,
}

impl Client {
  pub fn into_inner(&self) -> RustClient {
    RustClient::clone(&self.client)
  }
}

#[wasm_bindgen]
impl Client {
  /// Send a message to the Tangle.
  pub fn message(&self) -> MessageBuilder {
    MessageBuilder::new(self.clone())
  }
  /// Get a message from the Tangle.
  #[wasm_bindgen(js_name = getMessage)]
  pub fn get_message(&self) -> MessageGetter {
    MessageGetter::new(self.clone())
  }
  /// Generate addresses.
  #[wasm_bindgen(js_name = getAddresses)]
  pub fn get_addresses(&self, seed: String) -> AddressGetter {
    AddressGetter::new(self.clone(), seed)
  }
  /// Get an unspent address.
  #[wasm_bindgen(js_name = getUnspentAddress)]
  pub fn get_unspent_address(&self, seed: String) -> UnspentAddressGetter {
    UnspentAddressGetter::new(self.clone(), seed)
  }
  /// Get the account balance.
  #[wasm_bindgen(js_name = getBalance)]
  pub fn get_balance(&self, seed: String) -> BalanceGetter {
    BalanceGetter::new(self.clone(), seed)
  }

  /// GET /api/v1/addresses/{address} endpoint
  #[wasm_bindgen(js_name = getAddress)]
  pub fn get_address(&self) -> GetAddressBuilder {
    GetAddressBuilder::new(self.clone())
  }

  /// Get the nodeinfo.
  #[wasm_bindgen(js_name = getInfo)]
  pub fn get_info(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_info()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Get the nodeinfo.
  #[wasm_bindgen(js_name = getNodeInfo)]
  pub fn get_node_info(
    &self,
    url: String,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<Promise, JsValue> {
    Ok(future_to_promise(async move {
      RustClient::get_node_info(&url, jwt.clone(), to_basic_auth(&username, &password))
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Gets the network related information such as network_id and min_pow_score
  /// and if it's the default one, sync it first.
  #[wasm_bindgen(js_name = networkInfo)]
  pub fn get_network_info(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_network_info()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Gets the network id of the node we're connecting to.
  #[wasm_bindgen(js_name = getNetworkId)]
  pub fn get_network_id(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_network_id()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// returns the bech32_hrp
  #[wasm_bindgen(js_name = getBech32Hrp)]
  pub fn get_bech32_hrp(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_bech32_hrp()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// returns the bech32_hrp
  #[wasm_bindgen(js_name = getMinPowScore)]
  pub fn get_min_pow_score(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_min_pow_score()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Get the node health.
  #[wasm_bindgen(js_name = getHealth)]
  pub fn get_health(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_health()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Get tips.
  #[wasm_bindgen(js_name = getTips)]
  pub fn get_tips(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_tips()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Get peers.
  #[wasm_bindgen(js_name = getPeers)]
  pub fn get_peers(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_peers()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// GET /api/v1/outputs/{outputId} endpoint
  /// Find an output by its transaction_id and corresponding output_index.
  #[wasm_bindgen(js_name = getOutput)]
  pub fn get_output(&self, output_id: &str) -> Result<Promise, JsValue> {
    #[derive(Serialize)]
    struct OutputResponseDto {
      #[serde(rename = "messageId")]
      pub message_id: String,
      #[serde(rename = "transactionId")]
      pub transaction_id: String,
      #[serde(rename = "outputIndex")]
      pub output_index: u16,
      #[serde(rename = "isSpent")]
      pub is_spent: bool,
      pub address: String,
      pub amount: u64,
      pub kind: String,
      #[serde(rename = "ledgerIndex")]
      pub ledger_index: u32,
    }
    let client: Rc<RustClient> = self.client.clone();
    let output_id = UtxoInput::from_str(output_id).map_err(wasm_error)?;
    Ok(future_to_promise(async move {
      let response = client.get_output(&output_id).await.map_err(wasm_error)?;

      let (address, amount, kind) = match response.output {
        OutputDto::SignatureLockedSingle(o) => match o.address {
          AddressDto::Ed25519(addr) => (addr.address, o.amount, "SignatureLockedSingle".to_string()),
        },
        OutputDto::SignatureLockedDustAllowance(o) => match o.address {
          AddressDto::Ed25519(addr) => (addr.address, o.amount, "SignatureLockedDustAllowance".to_string()),
        },
        OutputDto::Treasury(o) => ("".to_string(), o.amount, "Treasury".to_string()),
      };
      let output = OutputResponseDto {
        message_id: response.message_id,
        transaction_id: response.transaction_id,
        output_index: response.output_index,
        is_spent: response.is_spent,
        address: client
          .hex_to_bech32(&address.to_string(), None)
          .await
          .map_err(wasm_error)?,
        amount,
        kind,
        ledger_index: response.ledger_index,
      };
      JsValue::from_serde(&output).map_err(wasm_error)
    }))
  }

  /// Find all messages by provided message IDs and/or indexation_keys.
  #[wasm_bindgen(js_name = findMessages)]
  pub fn find_messages(&self, indexation_keys: JsValue, message_ids: JsValue) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let indexation_keys: Vec<String> = indexation_keys.into_serde().map_err(wasm_error)?;
    let message_ids: Vec<String> = message_ids.into_serde().map_err(wasm_error)?;
    let message_ids = message_ids
      .into_iter()
      .map(|m| MessageId::from_str(&m).map_err(wasm_error))
      .collect::<Result<Vec<MessageId>, JsValue>>()?;
    Ok(future_to_promise(async move {
      client
        .find_messages(&indexation_keys, &message_ids)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Function to find inputs from addresses for a provided amount (useful for offline signing)
  #[wasm_bindgen(js_name = findInputs)]
  pub fn find_inputs(&self, addresses: JsValue, amount: u64) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let addresses: Vec<String> = addresses.into_serde().map_err(wasm_error)?;
    Ok(future_to_promise(async move {
      client
        .find_inputs(addresses, amount)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
  /// the request amount exceeds individual node limit.
  #[wasm_bindgen(js_name = findOutputs)]
  pub fn find_outputs(&self, outputs: JsValue, addresses: JsValue) -> Result<Promise, JsValue> {
    #[derive(Serialize)]
    struct OutputMetadataDto {
      /// Message ID of the output
      #[serde(rename = "messageId")]
      pub message_id: String,
      /// Transaction ID of the output
      #[serde(rename = "transactionId")]
      pub transaction_id: String,
      /// Output index.
      #[serde(rename = "outputIndex")]
      pub output_index: u16,
      /// Spend status of the output
      #[serde(rename = "isSpent")]
      pub is_spent: bool,
      /// Corresponding address
      pub address: String,
      /// Balance amount
      pub amount: u64,
      /// Output kind
      pub kind: String,
      #[serde(rename = "ledgerIndex")]
      pub ledger_index: u32,
    }
    let client: Rc<RustClient> = self.client.clone();
    let outputs: Vec<String> = outputs.into_serde().map_err(wasm_error)?;
    let addresses: Vec<String> = addresses.into_serde().map_err(wasm_error)?;
    let outputs = outputs
      .into_iter()
      .map(|o| UtxoInput::from_str(&o).map_err(wasm_error))
      .collect::<Result<Vec<UtxoInput>, JsValue>>()?;
    Ok(future_to_promise(async move {
      let outputs = client.find_outputs(&outputs, &addresses).await.map_err(wasm_error)?;
      let mut results = Vec::new();
      for output in outputs {
        let (address, amount, kind) = match output.output {
          OutputDto::SignatureLockedSingle(o) => match o.address {
            AddressDto::Ed25519(addr) => (addr.address, o.amount, "SignatureLockedSingle".to_string()),
          },
          OutputDto::SignatureLockedDustAllowance(o) => match o.address {
            AddressDto::Ed25519(addr) => (addr.address, o.amount, "SignatureLockedDustAllowance".to_string()),
          },
          OutputDto::Treasury(o) => ("".to_string(), o.amount, "Treasury".to_string()),
        };
        results.push(OutputMetadataDto {
          message_id: output.message_id,
          transaction_id: output.transaction_id,
          output_index: output.output_index,
          is_spent: output.is_spent,
          address: client
            .hex_to_bech32(&address.to_string(), None)
            .await
            .map_err(wasm_error)?,
          amount,
          kind,
          ledger_index: output.ledger_index,
        })
      }
      JsValue::from_serde(&results).map_err(wasm_error)
    }))
  }

  /// Return the balance in iota for the given addresses; No seed needed to do this since we are only checking and
  /// already know the addresses.
  #[wasm_bindgen(js_name = getAddressBalances)]
  pub fn get_address_balances(&self, addresses: JsValue) -> Result<Promise, JsValue> {
    #[derive(Serialize)]
    struct AddressBalanceDto {
      pub address: String,
      pub balance: u64,
      #[serde(rename = "dustAllowed")]
      pub dust_allowed: bool,
      #[serde(rename = "ledgerIndex")]
      pub ledger_index: u32,
    }

    let client: Rc<RustClient> = self.client.clone();
    let addresses: Vec<String> = addresses.into_serde().map_err(wasm_error)?;
    Ok(future_to_promise(async move {
      let balances = client.get_address_balances(&addresses).await.map_err(wasm_error)?;
      let mut results = Vec::new();
      for balance in balances {
        results.push(AddressBalanceDto {
          address: client
            .hex_to_bech32(&balance.address.to_string(), None)
            .await
            .map_err(wasm_error)?,
          balance: balance.balance,
          dust_allowed: balance.dust_allowed,
          ledger_index: balance.ledger_index,
        })
      }
      JsValue::from_serde(&results).map_err(wasm_error)
    }))
  }

  /// GET /api/v1/milestones/{index} endpoint
  /// Get the milestone by the given index.
  #[wasm_bindgen(js_name = getMilestone)]
  pub fn get_milestone(&self, index: u32) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_milestone(index)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// GET /api/v1/milestones/{index}/utxo-changes endpoint
  /// Get the milestone by the given index.
  #[wasm_bindgen(js_name = getMilestoneUtxoChanges)]
  pub fn get_milestone_utxo_changes(&self, index: u32) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_milestone_utxo_changes(index)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// GET /api/v1/receipts endpoint
  /// Get all receipts.
  #[wasm_bindgen(js_name = getReceipts)]
  pub fn get_receipts(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_receipts()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// GET /api/v1/receipts/{migratedAt} endpoint
  /// Get the receipts by the given milestone index.
  #[wasm_bindgen(js_name = getReceiptsMigratedAt)]
  pub fn get_receipts_migrated_at(&self, milestone_index: u32) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_receipts_migrated_at(milestone_index)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// GET /api/v1/treasury endpoint
  /// Get the treasury output.
  #[wasm_bindgen(js_name = getTreasury)]
  pub fn get_treasury(&self) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .get_treasury()
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// GET /api/v1/transactions/{transactionId}/included-message
  /// Returns the included message of the transaction.
  #[wasm_bindgen(js_name = getIncludedMessage)]
  pub fn get_included_message(&self, transaction_id: &str) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let transaction_id = TransactionId::from_str(transaction_id).map_err(wasm_error)?;
    Ok(future_to_promise(async move {
      client
        .get_included_message(&transaction_id)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Post message.
  #[wasm_bindgen(js_name = postMessage)]
  pub fn post_message(&self, message: JsValue) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let message: MessageDto = serde_json::from_value(message.into_serde().map_err(wasm_error)?).map_err(wasm_error)?;
    Ok(future_to_promise(async move {
      let mut parent_msg_ids = match message.parents.as_ref() {
        Some(parents) => {
          let mut parent_ids = Vec::new();
          for msg_id in parents {
            parent_ids.push(MessageId::from_str(msg_id).map_err(wasm_error)?)
          }
          parent_ids
        }
        None => client.get_tips().await.map_err(wasm_error)?,
      };
      parent_msg_ids.sort_unstable_by_key(|a| a.pack_new());
      parent_msg_ids.dedup();
      let network_id = client.get_network_id().await.map_err(wasm_error)?;
      let nonce_provider = client.get_pow_provider().await;
      let min_pow_score = client.get_min_pow_score().await.map_err(wasm_error)?;

      // XXX: error[E0277]: the trait bound `error::WasmError<'_>:
      //   std::convert::From<std::convert::Infallible>` is not satisfied
      // the trait `std::convert::From<std::convert::Infallible>` is not implemented for
      //   `error::WasmError<'_>`
      // note: required by a bound in `error::wasm_error`
      //    --> src/error.rs:12:6
      #[allow(clippy::needless_borrow)]
      let message = RustMessageBuilder::<ClientMiner>::new()
        .with_network_id(network_id)
        .with_parents(Parents::new(parent_msg_ids).map_err(wasm_error)?)
        .with_nonce_provider(nonce_provider, min_pow_score)
        .with_payload((&message.payload).try_into().map_err(wasm_error)?) // <- XXX
        .finish()
        .map_err(wasm_error)?;

      client
        .post_message(&message)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Retries (promotes or reattaches) a message for provided message id. Message should only be
  /// retried only if they are valid and haven't been confirmed for a while.
  #[wasm_bindgen]
  pub fn retry(&self, message_id: String) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .retry(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Only works in browser because of the timeouts
  /// Retries (promotes or reattaches) a message for provided message id until it's included (referenced by a
  /// milestone). Default interval is 5 seconds and max attempts is 40. Returns reattached messages
  #[wasm_bindgen(js_name = retryUntilIncluded)]
  pub fn retry_until_included(
    &self,
    message_id: String,
    interval: Option<u64>,
    max_attempts: Option<u64>,
  ) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .retry_until_included(
          &MessageId::from_str(&message_id).map_err(wasm_error)?,
          interval,
          max_attempts,
        )
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Reattaches messages for provided message id. Messages can be reattached only if they are valid and haven't been
  /// confirmed for a while.
  #[wasm_bindgen]
  pub fn reattach(&self, message_id: String) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .reattach(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Promotes a message. The method should validate if a promotion is necessary through get_message. If not, the
  /// method should error out and should not allow unnecessary promotions.
  #[wasm_bindgen]
  pub fn promote(&self, message_id: String) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .promote(&MessageId::from_str(&message_id).map_err(wasm_error)?)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Only works in browser because of the timeouts
  /// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
  /// Returns the address to which the funds got consolidated, if any were available
  #[wasm_bindgen(js_name = consolidateFunds)]
  pub fn consolidate_funds(
    &self,
    seed: String,
    account_index: usize,
    start_index: usize,
    end_index: usize,
  ) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    let seed = Seed::from_bytes(&hex::decode(&seed).map_err(wasm_error)?);
    Ok(future_to_promise(async move {
      client
        .consolidate_funds(&seed, account_index, start_index..end_index)
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Returns a parsed hex String from bech32.
  #[wasm_bindgen(js_name = bech32ToHex)]
  pub fn bech32_to_hex(&self, address: &str) -> Result<String, JsValue> {
    RustClient::bech32_to_hex(address).map_err(wasm_error)
  }

  /// Returns a parsed bech32 String from hex.
  #[wasm_bindgen(js_name = hexToBech32)]
  pub fn hex_to_bech32(&self, address: String, bech32: Option<String>) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .hex_to_bech32(&address, bech32.as_deref())
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Transforms a hex encoded public key to a bech32 encoded address
  #[wasm_bindgen(js_name = hexPublicKeyToBech32Address)]
  pub fn hex_public_key_to_bech32_address(
    &self,
    public_key: String,
    bech32: Option<String>,
  ) -> Result<Promise, JsValue> {
    let client: Rc<RustClient> = self.client.clone();
    Ok(future_to_promise(async move {
      client
        .hex_public_key_to_bech32_address(&public_key, bech32.as_deref())
        .await
        .map_err(wasm_error)
        .and_then(|res| JsValue::from_serde(&res).map_err(wasm_error))
    }))
  }

  /// Checks if a String is a valid bech32 encoded address.
  #[wasm_bindgen(js_name = isAddressValid)]
  pub fn is_address_valid(&self, address: String) -> bool {
    RustClient::is_address_valid(&address)
  }

  /// Generates a new mnemonic.
  #[wasm_bindgen(js_name = generateMnemonic)]
  pub fn generate_mnemonic(&self) -> Result<String, JsValue> {
    RustClient::generate_mnemonic().map_err(wasm_error)
  }

  /// Returns a hex encoded seed for a mnemonic.
  #[wasm_bindgen(js_name = mnemonicToHexSeed)]
  pub fn mnemonic_to_hex_seed(&self, mnemonic: &str) -> Result<String, JsValue> {
    RustClient::mnemonic_to_hex_seed(mnemonic).map_err(wasm_error)
  }

  /// Returns the message id from a provided message.
  #[wasm_bindgen(js_name = getMessageId)]
  pub fn get_message_id(&self, message: &str) -> Result<String, JsValue> {
    // Try BeeMessageDto and if it fails Message
    let message = match serde_json::from_str::<BeeMessageDto>(message) {
      Ok(message_dto) => Message::try_from(&message_dto).map_err(wasm_error)?,
      Err(_) => serde_json::from_str::<Message>(message).map_err(wasm_error)?,
    };
    Ok(message.id().0.to_string())
  }

  /// Returns the transaction id from a provided transaction payload.
  #[wasm_bindgen(js_name = getTransactionId)]
  pub fn get_transaction_id(&self, transaction: &str) -> Result<String, JsValue> {
    // Try TransactionPayloadDto and if it fails TransactionPayload
    let transaction = match serde_json::from_str::<TransactionPayloadDto>(transaction) {
      Ok(transaction_dto) => TransactionPayload::try_from(&transaction_dto).map_err(wasm_error)?,
      Err(_) => serde_json::from_str::<TransactionPayload>(transaction).map_err(wasm_error)?,
    };
    Ok(transaction.id().to_string())
  }

  /// Get essence hash
  #[wasm_bindgen(js_name = getEssenceHash)]
  pub fn get_essence_hash(&self, essence: &str) -> Result<String, JsValue> {
    let essence = serde_json::from_str::<Essence>(essence).map_err(wasm_error)?;
    let hashed_essence = essence.hash();
    Ok(hex::encode(hashed_essence))
  }
}
