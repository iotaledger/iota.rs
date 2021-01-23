// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::{Client, Message, MilestoneMetadata, NodeInfo, OutputMetadata, UTXOInput};
use iota::{
    Bech32Address as RustBech32Address, ClientMiner as RustClientMiner, MessageBuilder as RustMessageBuilder,
    MessageId as RustMessageId, UTXOInput as RustUTXOInput,
};
use pyo3::{exceptions, prelude::*};

use std::{
    convert::{From, Into},
    str::FromStr,
};

/// Full node API
#[pymethods]
impl Client {
    fn get_health(&self) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new()?;
        match rt.block_on(async { self.client.get_health().await }) {
            Err(err) => Err(PyErr::new::<exceptions::PyTypeError, _>(err.to_string())),
            Ok(healthy) => Ok(healthy),
        }
    }
    fn get_info(&self) -> NodeInfo {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let node_info = rt.block_on(async { self.client.get_info().await.unwrap() });
        node_info.into()
    }
    fn get_tips(&self) -> (String, String) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let tips = rt.block_on(async { self.client.get_tips().await.unwrap() });
        (tips.0.to_string(), tips.1.to_string())
    }
    fn post_message(&self, msg: Message) -> String {
        let mut msg_builder = RustMessageBuilder::<RustClientMiner>::new()
            .with_network_id(msg.network_id)
            .with_parent1(RustMessageId::from_str(&msg.parent1).unwrap())
            .with_parent2(RustMessageId::from_str(&msg.parent1).unwrap())
            .with_nonce_provider(self.client.get_pow_provider(), 4000f64);
        if let Some(payload) = msg.payload {
            msg_builder = msg_builder.with_payload(payload.into());
        }
        let msg = msg_builder.finish().unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message_id = rt.block_on(async { self.client.post_message(&msg).await.unwrap() });
        message_id.to_string()
    }
    fn get_output(&self, output_id: String) -> OutputMetadata {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let output_metadata = rt.block_on(async {
            self.client
                .get_output(&RustUTXOInput::from_str(&output_id).unwrap())
                .await
                .unwrap()
        });
        output_metadata.into()
    }
    fn get_address_balance(&self, address: &str) -> u64 {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.client
                .get_address()
                .balance(&RustBech32Address::from(address))
                .await
                .unwrap()
        })
    }
    fn get_address_outputs(&self, address: &str) -> Vec<UTXOInput> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let outputs = rt.block_on(async {
            self.client
                .get_address()
                .outputs(&RustBech32Address::from(address))
                .await
                .unwrap()
        });
        (*outputs)
            .to_vec()
            .iter()
            .map(|output| UTXOInput {
                transaction_id: output.output_id().transaction_id().as_ref().to_vec(),
                index: output.output_id().index(),
            })
            .collect()
    }
    fn find_outputs(&self, output_ids: Option<Vec<String>>, addresses: Option<Vec<String>>) -> Vec<OutputMetadata> {
        let output_ids: Vec<RustUTXOInput> = output_ids
            .unwrap_or(vec![])
            .iter()
            .map(|input| RustUTXOInput::from_str(input).unwrap_or_else(|_| panic!("invalid input: {}", input)))
            .collect();
        let addresses: Vec<RustBech32Address> = addresses
            .unwrap_or(vec![])
            .iter()
            .map(|address| RustBech32Address::from(&address[..]))
            .collect();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let output_metadata_vec =
            rt.block_on(async { self.client.find_outputs(&output_ids[..], &addresses[..]).await.unwrap() });
        output_metadata_vec
            .into_iter()
            .map(|metadata| metadata.into())
            .collect()
    }
    fn get_milestone(&self, index: u64) -> MilestoneMetadata {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let milestone_metadata = rt.block_on(async { self.client.get_milestone(index).await.unwrap() });
        milestone_metadata.into()
    }
}
