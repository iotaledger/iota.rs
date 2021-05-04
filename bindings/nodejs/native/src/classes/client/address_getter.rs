// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unnecessary_wraps)]
use iota_client::Seed;
use neon::prelude::*;

use std::ops::Range;

use super::{Api, ClientTask};

pub struct AddressGetter {
    client_id: String,
    seed: String,
    account_index: Option<usize>,
    range: Option<Range<usize>>,
    bech32_hrp: Option<String>,
    include_internal: bool,
}

declare_types! {
    pub class JsAddressGetter for AddressGetter {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(AddressGetter {
                client_id,
                seed,
                account_index: None,
                range: None,
                bech32_hrp: None,
                include_internal: false,
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

        method bech32Hrp(mut cx) {
            let bech32_hrp = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let find_bech32_hrp = &mut this.borrow_mut(&guard).bech32_hrp;
                find_bech32_hrp.replace(bech32_hrp);
            }

            Ok(cx.this().upcast())
        }

        method includeInternal(mut cx) {
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let include_internal = &mut this.borrow_mut(&guard).include_internal;
                *include_internal = true;
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
                    api: Api::GetAddresses {
                        seed: Seed::from_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")),
                        account_index: ref_.account_index,
                        range: ref_.range.clone(),
                        bech32_hrp: ref_.bech32_hrp.clone(),
                        include_internal: ref_.include_internal,
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

    }
}
