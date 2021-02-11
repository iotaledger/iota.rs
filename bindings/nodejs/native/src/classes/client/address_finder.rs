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
    bech32_hrp: Option<String>,
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
                bech32_hrp: None,
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
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));
                let client_task = ClientTask {
                    client_id: ref_.client_id.clone(),
                    api: Api::FindAddresses {
                        seed: Seed::fromcar_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")).expect("invalid seed"),
                        account_index: ref_.account_index,
                        range: ref_.range,
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
