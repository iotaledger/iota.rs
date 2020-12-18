// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{message::prelude::Address, Seed};
use neon::prelude::*;

use std::{
    num::NonZeroU64,
    sync::{Arc, Mutex},
};

use super::{parse_address, Api, ClientTask};

pub struct MessageSender(String);

pub struct IndexationSender {
    client_id: String,
    index: Arc<Mutex<Option<String>>>,
    data: Arc<Mutex<Option<Vec<u8>>>>,
}

pub struct ValueTransactionSender {
    client_id: String,
    seed: String,
    account_index: Arc<Mutex<Option<usize>>>,
    initial_address_index: Arc<Mutex<Option<usize>>>,
    outputs: Arc<Mutex<Vec<(Address, NonZeroU64)>>>,
}

declare_types! {
    pub class JsMessageSender for MessageSender {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            Ok(MessageSender(client_id))
        }

        method transaction(mut cx) {
            let seed = cx.argument::<JsString>(0)?;

            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsValueTransactionSender::new(&mut cx, vec![client_id, seed])?.upcast())
        }

        method indexation(mut cx) {
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsIndexationSender::new(&mut cx, vec![client_id])?.upcast())
        }
    }

    pub class JsIndexationSender for IndexationSender {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            Ok(IndexationSender {
                client_id,
                index: Default::default(),
                data: Default::default(),
            })
        }

        method index(mut cx) {
            let index = cx.argument::<JsString>(0)?.value();
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).index;
                let mut indexation = ref_.lock().unwrap();
                indexation.replace(index);
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
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).data;
                let mut data_ = ref_.lock().unwrap();
                data_.replace(data);
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
                    api: Api::SendIndexation {
                        index: ref_.index.lock().unwrap().clone(),
                        data: ref_.data.lock().unwrap().clone(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }

    pub class JsValueTransactionSender for ValueTransactionSender {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();

            // validate the seed
            Seed::from_ed25519_bytes(&hex::decode(&seed).expect("invalid seed hex")).expect("invalid seed");

            Ok(ValueTransactionSender {
                client_id,
                seed,
                account_index: Arc::new(Mutex::new(None)),
                initial_address_index: Arc::new(Mutex::new(None)),
                outputs: Arc::new(Mutex::new(vec![]))
            })
        }

        method accountIndex(mut cx) {
            let account_index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).account_index;
                let mut send_account_index = ref_.lock().unwrap();
                send_account_index.replace(account_index);
            }

            Ok(cx.this().upcast())
        }

        method initialAddressIndex(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).initial_address_index;
                let mut send_address_index = ref_.lock().unwrap();
                send_address_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method output(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();
            let address = parse_address(address).expect("invalid address");
            let value = cx.argument::<JsNumber>(1)?.value() as u64;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).outputs;
                let mut outputs = ref_.lock().unwrap();
                outputs.push((address, NonZeroU64::new(value).expect("value can't be zero")));
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
                    api: Api::SendTransfer {
                        seed: Seed::from_ed25519_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")).expect("invalid seed"),
                        account_index: *ref_.account_index.lock().unwrap(),
                        initial_address_index: *ref_.initial_address_index.lock().unwrap(),
                        outputs: (*ref_.outputs.lock().unwrap()).clone(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
