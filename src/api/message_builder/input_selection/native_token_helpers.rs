// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;

use bee_message::output::{Output, TokenId};
use primitive_types::U256;

use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
};

pub(crate) fn missing_native_tokens(
    inputs: &HashMap<TokenId, U256>,
    required: &HashMap<TokenId, U256>,
) -> Option<HashMap<TokenId, U256>> {
    let mut missing_native_tokens = HashMap::new();
    for (tokend_id, required_native_token_amount) in required {
        match inputs.get(tokend_id) {
            None => {
                missing_native_tokens.insert(*tokend_id, *required_native_token_amount);
            }
            Some(amount) => {
                if amount < required_native_token_amount {
                    missing_native_tokens.insert(*tokend_id, required_native_token_amount - amount);
                }
            }
        }
    }
    if missing_native_tokens.is_empty() {
        None
    } else {
        Some(missing_native_tokens)
    }
}

pub(crate) fn get_remainder_native_tokens(
    inputs: &HashMap<TokenId, U256>,
    required: &HashMap<TokenId, U256>,
) -> Option<HashMap<TokenId, U256>> {
    // inputs and required are switched
    missing_native_tokens(required, inputs)
}

// minted first, burned second
pub(crate) fn get_minted_and_burned_native_tokens(
    inputs: &[Output],
    outputs: &[Output],
) -> Result<(HashMap<TokenId, U256>, HashMap<TokenId, U256>)> {
    let mut minted_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    let mut burned_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    for output in outputs {
        if let Output::Foundry(output_foundry) = output {
            let mut initial_creation = true;
            for input in inputs {
                if let Output::Foundry(input_foundry) = input {
                    let token_id = TokenId::build(&output_foundry.id(), output_foundry.token_tag());
                    if output_foundry.id() == input_foundry.id() {
                        initial_creation = false;
                        match output_foundry
                            .circulating_supply()
                            .cmp(&input_foundry.circulating_supply())
                        {
                            Ordering::Greater => {
                                let minted_native_token_amount =
                                    output_foundry.circulating_supply() - input_foundry.circulating_supply();
                                match minted_native_tokens.entry(token_id) {
                                    Entry::Vacant(e) => {
                                        e.insert(minted_native_token_amount);
                                    }
                                    Entry::Occupied(mut e) => {
                                        *e.get_mut() += minted_native_token_amount;
                                    }
                                }
                            }
                            Ordering::Less => {
                                let burned_native_token_amount =
                                    input_foundry.circulating_supply() - output_foundry.circulating_supply();
                                match burned_native_tokens.entry(token_id) {
                                    Entry::Vacant(e) => {
                                        e.insert(burned_native_token_amount);
                                    }
                                    Entry::Occupied(mut e) => {
                                        *e.get_mut() += burned_native_token_amount;
                                    }
                                }
                            }
                            Ordering::Equal => {}
                        }
                    }
                }
            }
            // If we created the foundry with this transaction, then we need to add the circulating supply as minted
            // tokens
            if initial_creation {
                let token_id = TokenId::build(&output_foundry.id(), output_foundry.token_tag());
                let circulating_supply = output_foundry.circulating_supply();
                if circulating_supply != U256::from(0) {
                    match minted_native_tokens.entry(token_id) {
                        Entry::Vacant(e) => {
                            e.insert(circulating_supply);
                        }
                        Entry::Occupied(mut e) => {
                            *e.get_mut() += circulating_supply;
                        }
                    }
                }
            }
        }
    }
    Ok((minted_native_tokens, burned_native_tokens))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bee_message::output::TokenId;

    #[test]
    fn nativ_token() {
        let token_id_bytes: [u8; 38] =
            hex::decode("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap();
        let token_id = TokenId::from(token_id_bytes);

        // inputs == required
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(50));
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(50));

        assert_eq!(
            None,
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // no inputs
        let input_native_tokens = HashMap::new();
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(50));

        assert_eq!(
            Some(required_native_tokens.clone()),
            missing_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // no inputs used
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(50));
        let required_native_tokens = HashMap::new();

        assert_eq!(
            Some(input_native_tokens.clone()),
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // only a part of the inputs is used
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(50));
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(20));
        let mut remainder_native_tokens = HashMap::new();
        remainder_native_tokens.insert(token_id, U256::from(30));

        assert_eq!(
            Some(remainder_native_tokens),
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );

        // more amount than required
        let mut input_native_tokens = HashMap::new();
        input_native_tokens.insert(token_id, U256::from(20));
        let mut required_native_tokens = HashMap::new();
        required_native_tokens.insert(token_id, U256::from(50));
        let mut remainder_native_tokens = HashMap::new();
        remainder_native_tokens.insert(token_id, U256::from(30));

        assert_eq!(
            Some(remainder_native_tokens),
            missing_native_tokens(&input_native_tokens, &required_native_tokens)
        );
        assert_eq!(
            None,
            get_remainder_native_tokens(&input_native_tokens, &required_native_tokens)
        );
    }
}
