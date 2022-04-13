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
    use std::env;

    use dotenv::dotenv;

    use crate::{
        api::GetAddressesBuilderOptions as GenerateAddressesOptions,
        message_interface::{self, ClientMethod, MessageType, ResponseType},
        signing::{types::Network, GenerateAddressMetadata},
    };

    #[tokio::test]
    async fn generate_addresses() {
        // This test uses dotenv, which is not safe for use in production
        dotenv().unwrap();

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

        let signer = format!(
            "{{\"Mnemonic\":\"{}\"}}",
            &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap()
        );
        let options = GenerateAddressesOptions {
            coin_type: None,
            account_index: None,
            range: Some(std::ops::Range { start: 0, end: 10 }),
            bech32_hrp: Some("atoi".to_string()),
            metadata: Some(GenerateAddressMetadata {
                syncing: false,
                network: Network::Testnet,
            }),
        };
        let message = MessageType::CallClientMethod(ClientMethod::GenerateAddresses { signer, options });

        let response = message_interface::send_message(&message_handler, message).await;
        match response.response_type() {
            ResponseType::GeneratedAddresses(addresses) => println!("{:?}", serde_json::to_string(addresses).unwrap()),
            _ => panic!("Unexpected response type"),
        }
    }

    #[tokio::test]
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
        let signer = format!(
            "{{\"Mnemonic\":\"{}\"}}",
            &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap()
        );
        let options = GenerateAddressesOptions {
            coin_type: None,
            account_index: None,
            range: Some(std::ops::Range { start: 0, end: 10 }),
            bech32_hrp: Some("atoi".to_string()),
            metadata: Some(GenerateAddressMetadata {
                syncing: false,
                network: Network::Testnet,
            }),
        };

        let generate_addresses_message = MessageType::CallClientMethod(ClientMethod::GenerateAddresses {
            signer: signer.clone(),
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
            amount: amount,
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
            signer: Some(signer),
            options: Some(options),
        });

        let response = message_interface::send_message(&message_handler, generate_message).await;
        match response.response_type() {
            ResponseType::GeneratedMessage(transaction_data) => {
                println!("{}", serde_json::to_string(transaction_data).unwrap())
            }
            response_type => panic!("Unexpected response type: {:?}", response_type),
        }
    }
}
