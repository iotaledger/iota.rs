// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{BIP32Path, Seed};
use neon::prelude::*;

use std::sync::{Arc, Mutex};

use super::{Api, ClientTask};

pub struct UnspentAddressGetter {
    client_id: String,
    seed: String,
    path: Arc<Mutex<Option<BIP32Path>>>,
    index: Arc<Mutex<Option<usize>>>,
}

declare_types! {
    pub class JsUnspentAddressGetter for UnspentAddressGetter {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(UnspentAddressGetter {
                client_id,
                seed,
                path: Arc::new(Mutex::new(None)),
                index: Arc::new(Mutex::new(None)),
            })
        }

        method path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let path = BIP32Path::from_str(path.as_str()).expect("invalid bip32 path");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).path;
                let mut get_path = ref_.lock().unwrap();
                get_path.replace(path);
            }

            Ok(cx.this().upcast())
        }

        method index(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).index;
                let mut get_index = ref_.lock().unwrap();
                get_index.replace(index);
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
                    api: Api::GetUnspentAddress {
                        seed: Seed::from_ed25519_bytes(&hex::decode(&ref_.seed).expect("invalid seed hex")).expect("invalid seed"),
                        path: (*ref_.path.lock().unwrap()).clone(),
                        index: *ref_.index.lock().unwrap(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
