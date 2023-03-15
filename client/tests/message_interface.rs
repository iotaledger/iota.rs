// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "message_interface")]

use std::{env, str::FromStr};

use dotenv::dotenv;
use iota_client::{
    api::GetAddressesBuilderOptions as GenerateAddressesOptions,
    block::{block::dto::BlockDto, payload::transaction::dto::TransactionEssenceDto, BlockId},
    message_interface::{self, Message, Response},
    secret::SecretManagerDto,
};

#[tokio::test]
async fn generate_addresses() {
    let client_config = r#"{
            "nodes":[],
            "localPow":true,
            "fallbackToLocalPow": true
    }"#
    .to_string();
    let message_handler = message_interface::create_message_handler(Some(client_config)).unwrap();

    let secret_manager = format!(
        "{{\"mnemonic\":\"{}\"}}",
        "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river"
    );
    let options = GenerateAddressesOptions {
        coin_type: None,
        account_index: None,
        range: Some(std::ops::Range { start: 0, end: 10 }),
        internal: None,
        bech32_hrp: Some("atoi".to_string()),
        options: None,
    };
    let message = Message::GenerateAddresses {
        secret_manager: serde_json::from_str::<SecretManagerDto>(&secret_manager).unwrap(),
        options,
    };

    let response = message_handler.send_message(message).await;
    match response {
        Response::GeneratedAddresses(addresses) => println!("{:?}", serde_json::to_string(&addresses).unwrap()),
        _ => panic!("Unexpected response type"),
    };
}

#[tokio::test]
#[should_panic]
async fn build_and_post_block() {
    // This test uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create a client message handler with node health ignored
    let client_config = r#"{
            "nodes":[
                {
                    "url": "http://localhost:14265",
                    "auth": null,
                    "disabled": false
                }
            ],
            "ignoreNodeHealth": true,
            "localPow":true,
            "fallbackToLocalPow": true
        }"#
    .to_string();
    let message_handler = message_interface::create_message_handler(Some(client_config)).unwrap();

    // Generate addresses
    let secret_manager = format!(
        "{{\"mnemonic\":\"{}\"}}",
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap()
    );
    let options = GenerateAddressesOptions {
        coin_type: None,
        account_index: None,
        range: Some(std::ops::Range { start: 0, end: 10 }),
        internal: None,
        bech32_hrp: Some("atoi".to_string()),
        options: None,
    };

    let generate_addresses_message = Message::GenerateAddresses {
        secret_manager: serde_json::from_str(&secret_manager).unwrap(),
        options,
    };

    let response = message_handler.send_message(generate_addresses_message).await;
    let addresses = match response {
        Response::GeneratedAddresses(addresses) => addresses,
        _ => panic!("Unexpected response type"),
    };

    // Address to which we want to send the amount
    let address = "atoi1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6gek0qdq";
    let amount = 1_000_000;

    // Find inputs
    let find_inputs_message = Message::FindInputs {
        addresses: addresses.clone(),
        amount,
    };

    let response = message_handler.send_message(find_inputs_message).await;
    let inputs = match response {
        Response::Inputs(inputs) => inputs,
        response_type => panic!("Unexpected response type: {response_type:?}"),
    };

    // Generate block payload
    let inputs = serde_json::to_string(&inputs).unwrap();
    let output = format!("{{\"address\":\"{address}\", \"amount\":{amount}}}");

    let options = format!("{{\"inputs\": {inputs},\"output\": {output}}}");

    let options = serde_json::from_str(&options).unwrap();
    let build_and_post_block = Message::BuildAndPostBlock {
        secret_manager: Some(serde_json::from_str(&secret_manager).unwrap()),
        options: Some(options),
    };

    let response = message_handler.send_message(build_and_post_block).await;
    match response {
        Response::BlockIdWithBlock(block_id, block_data) => {
            println!("{block_id}: {}", serde_json::to_string(&block_data).unwrap());
        }
        response_type => panic!("Unexpected response type: {response_type:?}"),
    }
}

#[tokio::test]
async fn get_block_id() {
    let message_handler = message_interface::create_message_handler(None).unwrap();

    let block = r#"
        {
            "protocolVersion":2,
            "parents":
                [
                    "0x2881c4781c4126f2413a704ebdf8cd375b46007f8df0e32ee9158684ac7e307b",
                    "0xe1956a33d608cb2bcfd6adeb67fe56ed0f33fc5ffd157e28a71047ecc52b0314",
                    "0xecc442108b1f30b6208ea57d24d892a6bdbdd9eb068dd34640a4d38b3c757132",
                    "0xfad7cc342cfa1135f9c12e99f98ec1658ec178524d19bde7b4797d81cecf9ea6"
                ],
            "payload":
                {
                    "type":5,
                    "tag":"0x484f524e4554205370616d6d6572",

"data":"0x494f5441202d2041206e6577206461776e0a436f756e743a203030323330330a54696d657374616d703a20323032322d30342d32375431383a35343a30395a0a54697073656c656374696f6e3a203832c2b573"
                },
            "nonce":"22897"
        }"#;

    let block_dto: BlockDto = serde_json::from_str(block).unwrap();
    let message = Message::BlockId { block: block_dto };

    let response = message_handler.send_message(message).await;

    match response {
        Response::BlockId(block_id) => {
            assert_eq!(
                block_id,
                BlockId::from_str("0xbcd2b9feed097a7aa8b894cae5eaeb1d8f516a14af25aa6f7d8aa7e2604c406c").unwrap()
            );
        }
        response_type => panic!("Unexpected response type: {response_type:?}"),
    }
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold() {
    let message_handler = message_interface::create_message_handler(None).unwrap();

    let secret_manager_dto = r#"{"stronghold": {"password": "some_hopefully_secure_password", "snapshotPath": "teststronghold.stronghold"}}"#;
    let mnemonic = String::from(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast",
    );

    let message = Message::StoreMnemonic {
        secret_manager: serde_json::from_str(secret_manager_dto).unwrap(),
        mnemonic,
    };
    let _response = message_handler.send_message(message).await;

    // Generate an address with the stored mnemonic to verify that it's usable
    let options = GenerateAddressesOptions {
        coin_type: None,
        account_index: None,
        range: Some(std::ops::Range { start: 0, end: 1 }),
        internal: None,
        bech32_hrp: Some("rms".to_string()),
        options: None,
    };
    let message = Message::GenerateAddresses {
        secret_manager: serde_json::from_str(secret_manager_dto).unwrap(),
        options,
    };
    let response = message_handler.send_message(message).await;

    match response {
        Response::GeneratedAddresses(addresses) => {
            assert_eq!(
                addresses[0],
                "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a".to_string(),
            );
        }
        response_type => panic!("Unexpected response type: {response_type:?}"),
    }

    // Remove garbage after test, but don't care about the result
    std::fs::remove_file("teststronghold.stronghold").unwrap_or(());
}

#[tokio::test]
async fn hash_transaction_essence() {
    let message_handler = message_interface::create_message_handler(None).unwrap();

    let transaction_essence = r#"{
        "type": 1,
        "networkId": "8453507715857476362",
        "inputs": [
          {
            "type": 0,
            "transactionId": "0x6cb5226d9390afa41ee02306d429e1db532c617f86679a094519e8935571980f",
            "transactionOutputIndex": 0
          }
        ],
        "inputsCommitment": "0x2b9db8d620137f02061d207310ef8876cc43b78c2cb826936e0d64e13531bf85",
        "outputs": [
          {
            "type": 3,
            "amount": "1000000",
            "unlockConditions": [
              {
                "type": 0,
                "address": {
                  "type": 0,
                  "pubKeyHash": "0x00b35b7176c3db9cb4856df8703576ae19a563b44ea9bed069646cc6aa10d11f"
                }
              }
            ]
          }
        ]
      }"#;

    let essence_dto: TransactionEssenceDto = serde_json::from_str(transaction_essence).unwrap();
    let message = Message::HashTransactionEssence { essence: essence_dto };

    let response = message_handler.send_message(message).await;

    match response {
        Response::TransactionEssenceHash(essence_hash) => {
            assert_eq!(
                essence_hash,
                "0x4624e6735cadee3d6ee5d83f9f8848d454a434ddf2b891b885f664de87eee044"
            );
        }
        response_type => panic!("Unexpected response type: {response_type:?}"),
    }
}
