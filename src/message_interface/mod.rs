// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Message interface for bindings

mod client_method;
mod message;
mod message_handler;
mod message_type;
mod response;
mod response_type;

pub use client_method::ClientMethod;
pub use message::Message;
pub use message_handler::ClientMessageHandler;
pub use message_type::MessageType;
pub use response::Response;
pub use response_type::ResponseType;

use crate::{Client, Result};

use tokio::sync::mpsc::unbounded_channel;

/// Create message handler with client options
pub async fn create_message_handler(client: Client) -> Result<ClientMessageHandler> {
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
    use crate::{
        builder::ClientBuilder,
        message_interface::{self, client_method::GenerateAddressesOptions, ClientMethod, MessageType},
        signing::{types::Network, GenerateAddressMetadata},
    };
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn generate_addresses() {
        dotenv().unwrap();
        let client_config = r#"{
                "nodes":[],
                "localPow":true,
                "fallbackToLocalPow": true,
                "offline": true
         }"#;

         let signer = format!(
             "{{\"Mnemonic\":\"{}\"}}",
             &env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap()
         );
         
        let client = ClientBuilder::new()
            .from_json(client_config)
            .unwrap()
            .finish()
            .await
            .unwrap();

        let message_handler = message_interface::create_message_handler(client).await.unwrap();
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

        let message = MessageType::CallClientMethod {
            method: ClientMethod::GenerateAddresses {
                signer,
                options: Some(options),
            },
        };

        let response = message_interface::send_message(&message_handler, message).await;
        println!("-> {}", serde_json::to_string(&response).unwrap());
    }
}
