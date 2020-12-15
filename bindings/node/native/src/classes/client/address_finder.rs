// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::Seed;
use neon::prelude::*;

use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

pub struct AddressFinder {
    client_id: String,
    seed: String,
    account_index: Arc<Mutex<Option<usize>>>,
    range: Arc<Mutex<Option<Range<usize>>>>,
}

declare_types! {
    pub class JsAddressFinder for AddressFinder {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(AddressFinder {
                client_id,
                seed,
                account_index: Arc::new(Mutex::new(None)),
                range: Arc::new(Mutex::new(None)),
            })
        }

        method accountIndex(mut cx) {
            let account_index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).account_index;
                let mut find_account_index = ref_.lock().unwrap();
                find_account_index.replace(account_index);
            }

            Ok(cx.this().upcast())
        }

        method range(mut cx) {
            let start = cx.argument::<JsNumber>(0)?.value() as usize;
            let end = cx.argument::<JsNumber>(1)?.value() as usize;
            let range = Range { start, end };
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).range;
                let mut find_range = ref_.lock().unwrap();
                find_range.replace(range);
            }

            Ok(cx.this().upcast())
        }

        method get(mut cx) {
            let addresses_json = {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));

                let seed = Seed::from_ed25519_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")).expect("invalid seed");

                let client = crate::get_client(&ref_.client_id);
                let client = client.read().unwrap();
                let mut getter = client.find_addresses(&seed);

                let account_index = &*ref_.account_index.lock().unwrap();
                if let Some(account_index) = account_index {
                    getter = getter.account_index(*account_index);
                }
                if let Some(range) = &*ref_.range.lock().unwrap() {
                    getter = getter.range(range.clone());
                }
                getter.get().map(|addresses| {
                    let addresses: Vec<(String, bool)> = addresses.iter().map(|(a, i)| (a.to_bech32(), *i)).collect();
                    serde_json::to_string(&addresses).unwrap()
                })
            };

            match addresses_json {
                Ok(addresses) => Ok(cx.string(addresses).upcast()),
                Err(e) => cx.throw_error(e.to_string()),
            }
        }
    }
}
