// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Message interface for bindings

mod client_method;
mod message;
mod message_handler;
mod message_type;
mod response;
mod response_type;

use tokio::sync::mpsc::unbounded_channel;

pub use self::{
    client_method::ClientMethod, message::Message, message_handler::ClientMessageHandler, message_type::MessageType,
    response::Response, response_type::ResponseType,
};
use crate::{ClientBuilder, Result};

/// Create message handler with client options
pub async fn create_message_handler(client_config: Option<String>) -> Result<ClientMessageHandler> {
    let client = match client_config {
        Some(options) => ClientBuilder::new().from_json(&options)?.finish().await?,
        None => ClientBuilder::new().finish().await?,
    };
    Ok(ClientMessageHandler::with_client(client))
}

/// Send message to message handler
pub async fn send_message(handle: &ClientMessageHandler, message_type: MessageType) -> Response {
    let (message_tx, mut message_rx) = unbounded_channel();
    let message = Message::new(message_type, message_tx);
    handle.handle(message).await;
    message_rx.recv().await.unwrap()
}

#[cfg(test)]
mod tests {
    use std::{env, str::FromStr};

    use bee_message::{MessageDto, MessageId};
    use dotenv::dotenv;

    use crate::{
        api::GetAddressesBuilderOptions as GenerateAddressesOptions,
        message_interface::{self, ClientMethod, MessageType, ResponseType},
        secret::{types::Network, GenerateAddressMetadata, SecretManagerDto},
    };

    #[tokio::test]
    async fn generate_addresses() {
        let client_config = r#"{
                "nodes":[],
                "localPow":true,
                "fallbackToLocalPow": true,
                "offline": true
         }"#
        .to_string();
        let message_handler = message_interface::create_message_handler(Some(client_config))
            .await
            .unwrap();

        let secret_manager = format!(
            "{{\"Mnemonic\":\"{}\"}}",
            "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river"
        );
        let options = GenerateAddressesOptions {
            coin_type: None,
            account_index: None,
            range: Some(std::ops::Range { start: 0, end: 10 }),
            internal: None,
            bech32_hrp: Some("atoi".to_string()),
            metadata: Some(GenerateAddressMetadata {
                syncing: false,
                network: Network::Testnet,
            }),
        };
        let message = MessageType::CallClientMethod(ClientMethod::GenerateAddresses {
            secret_manager: serde_json::from_str::<SecretManagerDto>(&secret_manager).unwrap(),
            options,
        });

        let response = message_interface::send_message(&message_handler, message).await;
        match response.response_type() {
            ResponseType::GeneratedAddresses(addresses) => println!("{:?}", serde_json::to_string(addresses).unwrap()),
            _ => panic!("Unexpected response type"),
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn generate_message() {
        // This test uses dotenv, which is not safe for use in production
        dotenv().ok();

        // Create a client message handler with node sync disabled
        let client_config = r#"{
            "nodes":[
                {
                    "url": "http://localhost:14265",
                    "auth": null,
                    "disabled": false
                }
            ],
            "nodeSyncEnabled": false,
            "localPow":true,
            "fallbackToLocalPow": true,
            "offline": true
        }"#
        .to_string();
        let message_handler = message_interface::create_message_handler(Some(client_config))
            .await
            .unwrap();

        // Generate addresses
        let secret_manager = format!(
            "{{\"Mnemonic\":\"{}\"}}",
            &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap()
        );
        let options = GenerateAddressesOptions {
            coin_type: None,
            account_index: None,
            range: Some(std::ops::Range { start: 0, end: 10 }),
            internal: None,
            bech32_hrp: Some("atoi".to_string()),
            metadata: Some(GenerateAddressMetadata {
                syncing: false,
                network: Network::Testnet,
            }),
        };

        let generate_addresses_message = MessageType::CallClientMethod(ClientMethod::GenerateAddresses {
            secret_manager: serde_json::from_str(&secret_manager).unwrap(),
            options,
        });

        let response = message_interface::send_message(&message_handler, generate_addresses_message).await;
        let addresses = match response.response_type() {
            ResponseType::GeneratedAddresses(addresses) => addresses,
            _ => panic!("Unexpected response type"),
        };

        // Address to which we want to send the amount
        let address = "atoi1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6gek0qdq";
        let amount = 1_000_000;

        // Find inputs
        let find_inputs_message = MessageType::CallClientMethod(ClientMethod::FindInputs {
            addresses: addresses.to_vec(),
            amount,
        });

        let response = message_interface::send_message(&message_handler, find_inputs_message).await;
        let inputs = match response.response_type() {
            ResponseType::Inputs(inputs) => inputs,
            response_type => panic!("Unexpected response type: {:?}", response_type),
        };

        // Generate message payload
        let inputs = serde_json::to_string(inputs).unwrap();
        let output = format!("{{\"address\":\"{}\", \"amount\":{}}}", address, amount);

        let options = format!("{{\"inputs\": {inputs},\"output\": {output}}}");

        let options = serde_json::from_str(&options).unwrap();
        let generate_message = MessageType::CallClientMethod(ClientMethod::GenerateMessage {
            secret_manager: Some(serde_json::from_str(&secret_manager).unwrap()),
            options: Some(options),
        });

        let response = message_interface::send_message(&message_handler, generate_message).await;
        match response.response_type() {
            ResponseType::GeneratedMessage(message_data) => {
                println!("{}", serde_json::to_string(message_data).unwrap())
            }
            response_type => panic!("Unexpected response type: {:?}", response_type),
        }
    }

    #[tokio::test]
    async fn get_message_id() {
        let client_config = r#"{"offline": true}"#.to_string();
        let message_handler = message_interface::create_message_handler(Some(client_config))
            .await
            .unwrap();

        let message = r#"
        {
            "protocolVersion":2,
            "parentMessageIds":
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

        let message_dto: MessageDto = serde_json::from_str(message).unwrap();
        let message_type = MessageType::CallClientMethod(ClientMethod::MessageId { message: message_dto });

        let response = message_interface::send_message(&message_handler, message_type).await;

        match response.response_type() {
            ResponseType::MessageId(message_id) => {
                assert_eq!(
                    *message_id,
                    MessageId::from_str("0xbcd2b9feed097a7aa8b894cae5eaeb1d8f516a14af25aa6f7d8aa7e2604c406c").unwrap()
                );
            }
            response_type => panic!("Unexpected response type: {:?}", response_type),
        }
    }

    #[cfg(feature = "stronghold")]
    #[tokio::test]
    async fn stronghold() {
        let client_config = r#"{"offline": true}"#.to_string();
        let message_handler = message_interface::create_message_handler(Some(client_config))
            .await
            .unwrap();

        let secret_manager_dto = r#"{"Stronghold": {"password": "some_hopefully_secure_password", "snapshotPath": "teststronghold.stronghold"}}"#;
        let mnemonic = String::from(
            "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast",
        );

        let message_type = MessageType::CallClientMethod(ClientMethod::StoreMnemonic {
            secret_manager: serde_json::from_str(secret_manager_dto).unwrap(),
            mnemonic,
        });
        let _response = message_interface::send_message(&message_handler, message_type).await;

        // Generate an address with the stored mnemonic to verify that it's usable
        let options = GenerateAddressesOptions {
            coin_type: None,
            account_index: None,
            range: Some(std::ops::Range { start: 0, end: 1 }),
            internal: None,
            bech32_hrp: Some("rms".to_string()),
            metadata: None,
        };
        let message = MessageType::CallClientMethod(ClientMethod::GenerateAddresses {
            secret_manager: serde_json::from_str(secret_manager_dto).unwrap(),
            options,
        });
        let response = message_interface::send_message(&message_handler, message).await;

        match response.response_type() {
            ResponseType::GeneratedAddresses(addresses) => {
                assert_eq!(
                    addresses[0],
                    "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a".to_string(),
                );
            }
            response_type => panic!("Unexpected response type: {:?}", response_type),
        }

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file("teststronghold.stronghold").unwrap_or(());
    }
}
