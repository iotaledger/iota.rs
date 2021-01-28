// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::Seed;
use neon::prelude::*;

use std::ops::Range;

pub struct AddressFinder {
    client_id: String,
    seed: String,
    account_index: Option<usize>,
    range: Option<Range<usize>>,
}

declare_types! {
    pub class JsAddressFinder for AddressFinder {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(AddressFinder {
                client_id,
                seed,
                account_index: None,
                range: None,
            })
        }

        method accountIndex(mut cx) {
            let account_index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let find_account_index = &mut this.borrow_mut(&guard).account_index;
                find_account_index.replace(account_index);
            }

            Ok(cx.this().upcast())
        }

        method range(mut cx) {
            let start = cx.argument::<JsNumber>(0)?.value() as usize;
            let end = cx.argument::<JsNumber>(1)?.value() as usize;
            let range = Range { start, end };
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let find_range = &mut this.borrow_mut(&guard).range;
                find_range.replace(range);
            }

            Ok(cx.this().upcast())
        }

        method get(mut cx) {
            let addresses_json = {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &this.borrow(&guard);

                let seed = Seed::from_ed25519_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")).expect("invalid seed");

                let client = crate::get_client(&ref_.client_id);
                let client = client.read().unwrap();
                let mut getter = client.find_addresses(&seed);

                if let Some(account_index) = &ref_.account_index {
                    getter = getter.with_account_index(*account_index);
                }
                if let Some(range) = &ref_.range {
                    getter = getter.with_range(range.clone());
                }
                getter.get_all().map(|addresses| {
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
