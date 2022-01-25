// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::{
        address::search_address,
        message_builder::DUST_THRESHOLD,
        types::{AddressIndexRecorder, OutputWrapper},
        ClientMessageBuilder, ADDRESS_GAP_RANGE,
    },
    node_api::indexer_api::query_parameters::{QueryParameter, QueryParameters},
    Error, Result,
};

use bee_message::{
    address::Address,
    input::{Input, UtxoInput, INPUT_COUNT_MAX},
    output::{ExtendedOutput, Output},
};
use bee_rest_api::types::dtos::OutputDto;

// Searches inputs for an amount which a user wants to spend, also checks that it doesn't create dust
pub(crate) async fn get_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    total_to_spend: u64,
) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
    let mut outputs = Vec::new();
    let mut inputs_for_essence = Vec::new();
    let mut outputs_for_essence = Vec::new();
    let mut address_index_recorders = Vec::new();
    let mut total_already_spent = 0;
    let account_index = message_builder.account_index.unwrap_or(0);
    let mut gap_index = message_builder.initial_address_index.unwrap_or(0);
    let mut empty_address_count: u64 = 0;
    'input_selection: loop {
        // Get the addresses in the BIP path/index ~ path/index+20
        let addresses = message_builder
            .client
            .get_addresses(
                message_builder
                    .signer
                    .as_ref()
                    .ok_or(crate::Error::MissingParameter("signer"))?,
            )
            .with_account_index(account_index)
            .with_range(gap_index..gap_index + ADDRESS_GAP_RANGE)
            .get_all()
            .await?;
        // Have public and internal addresses with the index ascending ordered
        let mut public_and_internal_addresses = Vec::new();
        for index in 0..addresses.public.len() {
            public_and_internal_addresses.push((addresses.public[index].clone(), false));
            public_and_internal_addresses.push((addresses.internal[index].clone(), true));
        }

        // For each address, get the address outputs
        let mut address_index = gap_index;
        for (index, (str_address, internal)) in public_and_internal_addresses.iter().enumerate() {
            let output_ids = crate::node_api::indexer_api::routes::output_ids(
                &message_builder.client,
                QueryParameters::new(vec![QueryParameter::Address(str_address.to_string())]),
            )
            .await?;

            let address_outputs =
                crate::node_api::core_api::get_outputs(message_builder.client.clone(), output_ids).await?;

            // If there are more than 20 (ADDRESS_GAP_RANGE) consecutive empty addresses, then we stop
            // looking up the addresses belonging to the seed. Note that we don't
            // really count the exact 20 consecutive empty addresses, which is
            // unnecessary. We just need to check the address range,
            // (index * ADDRESS_GAP_RANGE, index * ADDRESS_GAP_RANGE + ADDRESS_GAP_RANGE), where index is
            // natural number, and to see if the outputs are all empty.
            if address_outputs.is_empty() {
                // Accumulate the empty_address_count for each run of output address searching
                empty_address_count += 1;
            } else {
                // Reset counter if there is an output
                empty_address_count = 0;
            }

            // We store output responses locally in outputs and after each output we sort
            // them and try to get enough inputs for the transaction, so we don't request more
            // outputs than we need
            for output in address_outputs.into_iter() {
                if !output.is_spent {
                    let (amount, _) = ClientMessageBuilder::get_output_amount_and_address(&output.output)?;

                    let output_wrapper = OutputWrapper {
                        output,
                        address_index,
                        internal: *internal,
                        amount,
                        address: str_address.clone(),
                    };
                    match output_wrapper.output.output {
                        OutputDto::Extended(_) => outputs.push(output_wrapper),
                        OutputDto::Treasury(_) => {}
                        // todo: add other outputs
                        _ => unimplemented!(),
                    };

                    // Order outputs descending, so that as few inputs as necessary are used
                    outputs.sort_by(|l, r| r.amount.cmp(&l.amount));

                    for (_offset, output_wrapper) in outputs
                        .iter()
                        // Max inputs is 127
                        .take(INPUT_COUNT_MAX.into())
                        .enumerate()
                    {
                        total_already_spent += output_wrapper.amount;
                        let address_index_record = ClientMessageBuilder::create_address_index_recorder(
                            account_index,
                            output_wrapper.address_index,
                            output_wrapper.internal,
                            &output_wrapper.output,
                            str_address.to_owned(),
                        )?;
                        inputs_for_essence.push(address_index_record.input.clone());
                        address_index_recorders.push(address_index_record);
                        // Break if we have enough funds and don't create dust for the remainder
                        if total_already_spent == total_to_spend
                            || total_already_spent >= total_to_spend + DUST_THRESHOLD
                        {
                            let remaining_balance = total_already_spent - total_to_spend;
                            // Output possible remaining tokens back to the original address
                            if remaining_balance != 0 {
                                outputs_for_essence.push(Output::Extended(ExtendedOutput::new(
                                    Address::try_from_bech32(&output_wrapper.address)?,
                                    remaining_balance,
                                )));
                            }
                            break 'input_selection;
                        }
                    }
                    // We need to cleare all gathered records if we haven't reached the total amount we need in this
                    // iteration.
                    inputs_for_essence.clear();
                    outputs_for_essence.clear();
                    address_index_recorders.clear();
                    total_already_spent = 0;
                }
            }

            // if we just processed an even index, increase the address index
            // (because the list has public and internal addresses)
            if index % 2 == 1 {
                address_index += 1;
            }
        }
        gap_index += ADDRESS_GAP_RANGE;
        // The gap limit is 20 and use reference 40 here because there's public and internal addresses
        if empty_address_count >= (ADDRESS_GAP_RANGE * 2) as u64 {
            let inputs_balance = outputs.iter().fold(0, |acc, output| acc + output.amount);
            let inputs_amount = outputs.len();
            if inputs_balance >= total_to_spend && inputs_amount > INPUT_COUNT_MAX.into() {
                return Err(Error::ConsolidationRequired(inputs_amount));
            } else if inputs_balance > total_to_spend {
                return Err(Error::DustError(format!(
                    "Transaction would create a dust output with {}i",
                    inputs_balance - total_to_spend
                )));
            } else {
                return Err(Error::NotEnoughBalance(inputs_balance, total_to_spend));
            }
        }
    }

    Ok((inputs_for_essence, outputs_for_essence, address_index_recorders))
}

// If custom inputs are provided we check if they are unspent, get the balance and search the address for it
pub(crate) async fn get_custom_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    inputs: &[UtxoInput],
    total_to_spend: u64,
) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
    let mut inputs_for_essence = Vec::new();
    let mut outputs_for_essence = Vec::new();
    let mut address_index_recorders = Vec::new();
    let mut remainder_address_balance: (Option<Address>, u64) = (None, 0);
    let mut total_already_spent = 0;
    let account_index = message_builder.account_index.unwrap_or(0);
    for input in inputs {
        // Only add unspent outputs
        if let Ok(output) = message_builder.client.get_output(input.output_id()).await {
            if !output.is_spent {
                let (output_amount, output_address) =
                    ClientMessageBuilder::get_output_amount_and_address(&output.output)?;

                total_already_spent += output_amount;
                let bech32_hrp = message_builder.client.get_bech32_hrp().await?;
                let (address_index, internal) = match message_builder.signer {
                    Some(signer) => {
                        search_address(
                            signer,
                            &bech32_hrp,
                            account_index,
                            message_builder.input_range.clone(),
                            &output_address,
                        )
                        .await?
                    }
                    None => (0, false),
                };

                let address_index_record = ClientMessageBuilder::create_address_index_recorder(
                    account_index,
                    address_index,
                    internal,
                    &output,
                    output_address.to_bech32(&bech32_hrp),
                )?;
                inputs_for_essence.push(address_index_record.input.clone());
                address_index_recorders.push(address_index_record);
                // Output the remaining tokens back to the original address
                if total_already_spent > total_to_spend {
                    let remaining_balance = total_already_spent - total_to_spend;
                    // Keep track of remaining balance, we don't add an output here, because we could have
                    // multiple inputs from the same address, which would create multiple outputs with the
                    // same address, which is not allowed
                    remainder_address_balance = (Some(output_address), remaining_balance);
                }
            }
        }
    }
    // Add output from remaining balance of custom inputs if necessary
    if let Some(address) = remainder_address_balance.0 {
        outputs_for_essence.push(Output::Extended(ExtendedOutput::new(
            address,
            remainder_address_balance.1,
        )));
    }

    if total_already_spent < total_to_spend {
        return Err(Error::NotEnoughBalance(total_already_spent, total_to_spend));
    }

    Ok((inputs_for_essence, outputs_for_essence, address_index_recorders))
}
