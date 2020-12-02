use iota::{BIP32Path, Seed};
use neon::prelude::*;

use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

pub struct AddressFinder {
    client_id: String,
    seed: String,
    path: Arc<Mutex<Option<BIP32Path>>>,
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
                path: Arc::new(Mutex::new(None)),
                range: Arc::new(Mutex::new(None)),
            })
        }

        method path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let path = BIP32Path::from_str(path.as_str()).expect("invalid bip32 path");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).path;
                let mut find_path = ref_.lock().unwrap();
                find_path.replace(path);
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

                let path = &*ref_.path.lock().unwrap();
                if let Some(path) = path {
                    getter = getter.path(&path);
                }
                if let Some(range) = &*ref_.range.lock().unwrap() {
                    getter = getter.range(range.clone());
                }
                getter.get().map(|addresses| {
                    let addresses: Vec<String> = addresses.iter().map(|a| a.to_bech32()).collect();
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
