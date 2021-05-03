// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::Seed;
use neon::prelude::*;

use super::{Api, ClientTask};

pub struct BalanceGetter {
    client_id: String,
    seed: String,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
    gap_limit: Option<usize>,
}

declare_types! {
    pub class JsBalanceGetter for BalanceGetter {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(BalanceGetter {
                client_id,
                seed,
                account_index: None,
                initial_address_index: None,
                gap_limit: None,
            })
        }

        method accountIndex(mut cx) {
            let account_index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let get_account_index = &mut this.borrow_mut(&guard).account_index;
                get_account_index.replace(account_index);
            }

            Ok(cx.this().upcast())
        }

        method initialAddressIndex(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let get_address_index = &mut this.borrow_mut(&guard).initial_address_index;
                get_address_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method gapLimit(mut cx) {
            let amount = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let gap_limit = &mut this.borrow_mut(&guard).gap_limit;
                gap_limit.replace(amount);
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
                        seed: Seed::from_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")),
                        account_index: ref_.account_index,
                        initial_address_index: ref_.initial_address_index,
                        gap_limit: ref_.gap_limit,
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
