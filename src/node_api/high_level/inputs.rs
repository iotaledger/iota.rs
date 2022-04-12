// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Result};
use bee_message::{
    input::Input,
    payload::{
        transaction::{TransactionEssence, TransactionId},
        Payload,
    },
};
use bee_rest_api::types::responses::OutputResponse;

/// Builder to get the inputs for the given transaction id.
pub struct GetTransactionInputsBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetTransactionInputsBuilder<'a> {
    /// Create builder.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Consume the builder and get the inputs for the given transaction id.
    pub async fn transaction_inputs(self, transaction_id: &TransactionId) -> Result<Vec<OutputResponse>> {
        let message = crate::node_api::core_api::routes::get_included_message(self.client, transaction_id).await?;

        let inputs = match message.payload() {
            Some(Payload::Transaction(t)) => match t.essence() {
                TransactionEssence::Regular(e) => e.inputs(),
            },
            _ => {
                unreachable!()
            }
        };

        let input_ids = inputs
            .iter()
            .map(|i| match i {
                Input::Utxo(input) => *input.output_id(),
                _ => {
                    unreachable!()
                }
            })
            .collect();

        crate::node_api::core_api::get_outputs(self.client, input_ids).await
    }
}
