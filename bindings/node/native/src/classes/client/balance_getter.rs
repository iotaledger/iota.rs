// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::Seed;
use neon::prelude::*;

use std::sync::{Arc, Mutex};

use super::{Api, ClientTask};

pub struct BalanceGetter {
    client_id: String,
    seed: String,
    account_index: Arc<Mutex<Option<usize>>>,
    initial_address_index: Arc<Mutex<Option<usize>>>,
}

declare_types! {
    pub class JsBalanceGetter for BalanceGetter {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(BalanceGetter {
                client_id,
                seed,
                account_index: Arc::new(Mutex::new(None)),
                initial_address_index: Arc::new(Mutex::new(None)),
            })
        }

        method accountIndex(mut cx) {
            let account_index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).account_index;
                let mut get_account_index = ref_.lock().unwrap();
                get_account_index.replace(account_index);
            }

            Ok(cx.this().upcast())
        }

        method initialAddressIndex(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).initial_address_index;
                let mut get_address_index = ref_.lock().unwrap();
                get_address_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method get(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));
                let client_task = ClientTask {
                    client_id: ref_.client_id.clone(),
                    api: Api::GetBalance {
                        seed: Seed::from_ed25519_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")).expect("invalid seed"),
                        account_index: (*ref_.account_index.lock().unwrap()).clone(),
                        initial_address_index: *ref_.initial_address_index.lock().unwrap(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
