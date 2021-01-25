// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Address, MessageId, Seed, TransactionId, UTXOInput};
use neon::prelude::*;

use super::{parse_address, Api, ClientTask};

use std::{ops::Range, str::FromStr};

pub struct MessageSender {
    client_id: String,
    index: Option<String>,
    data: Option<Vec<u8>>,
    parent: Option<MessageId>,
    seed: Option<String>,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
    inputs: Vec<UTXOInput>,
    input_range: Range<usize>,
    outputs: Vec<(Address, u64)>,
}

declare_types! {
    pub class JsMessageSender for MessageSender {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            Ok(MessageSender {
                client_id,
                index: None,
                data: None,
                parent: None,
                seed: None,
                account_index: None,
                initial_address_index:None,
                inputs: Vec::new(),
                input_range: 0..100,
                outputs: Vec::new(),
            })
        }

        method seed(mut cx) {
            let seed = cx.argument::<JsString>(0)?.value();

            // validate the seed
            Seed::from_ed25519_bytes(&hex::decode(&seed).expect("invalid seed hex")).expect("invalid seed");

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let send_seed = &mut this.borrow_mut(&guard).seed;
                send_seed.replace(seed);
            }

            Ok(cx.this().upcast())
        }

        method index(mut cx) {
            let index = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let send_index = &mut this.borrow_mut(&guard).index;
                send_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method data(mut cx) {
            let mut data: Vec<u8> = vec![];
            let data_js_array = cx.argument::<JsArray>(0)?;
            let js_data: Vec<Handle<JsValue>> = data_js_array.to_vec(&mut cx)?;
            for value in js_data {
                let value: Handle<JsNumber> = value.downcast_or_throw(&mut cx)?;
                data.push(value.value() as u8);
            }

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let data_ = &mut this.borrow_mut(&guard).data;
                data_.replace(data);
            }

            Ok(cx.this().upcast())
        }

        method parent(mut cx) {
            let parent = cx.argument::<JsString>(0)?.value();
            let parent = MessageId::from_str(&parent).expect("invalid parent message id");
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let send_parent = &mut this.borrow_mut(&guard).parent;
                send_parent.replace(parent);
            }

            Ok(cx.this().upcast())
        }

        method accountIndex(mut cx) {
            let account_index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let send_account_index = &mut this.borrow_mut(&guard).account_index;
                send_account_index.replace(account_index);
            }

            Ok(cx.this().upcast())
        }

        method initialAddressIndex(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let send_address_index = &mut this.borrow_mut(&guard).initial_address_index;
                send_address_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method output(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();
            let address = parse_address(address).expect("invalid address");
            let value = cx.argument::<JsNumber>(1)?.value() as u64;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let outputs = &mut this.borrow_mut(&guard).outputs;
                outputs.push((address, value));
            }

            Ok(cx.this().upcast())
        }

        method input(mut cx) {
            let transaction_id = cx.argument::<JsString>(0)?.value();
            let transaction_id = TransactionId::from_str(&transaction_id).expect("invalid transaction id");
            let index = cx.argument::<JsNumber>(1)?.value() as u16;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let inputs = &mut this.borrow_mut(&guard).inputs;
                inputs.push(UTXOInput::new(transaction_id, index).expect("invalid UTXO input"));
            }

            Ok(cx.this().upcast())
        }

        method inputRange(mut cx){
            let start = cx.argument::<JsNumber>(0)?.value() as usize;
            let end = cx.argument::<JsNumber>(1)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let input_range = &mut this.borrow_mut(&guard).input_range;
                *input_range = start..end;
            }
            Ok(cx.this().upcast())
        }

        method submit(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));
                let client_task = ClientTask {
                    client_id: ref_.client_id.clone(),
                    api: Api::Send {
                        seed: ref_.seed.as_ref().map(|seed| Seed::from_ed25519_bytes(&hex::decode(&seed).expect("invalid seed hex")).expect("invalid seed")),
                        index: ref_.index.clone(),
                        data: ref_.data.clone(),
                        parent: ref_.parent,
                        account_index: ref_.account_index,
                        initial_address_index: ref_.initial_address_index,
                        inputs: ref_.inputs.clone(),
                        outputs: ref_.outputs.clone(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
