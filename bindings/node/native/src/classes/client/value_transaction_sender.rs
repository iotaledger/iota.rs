use iota::{message::prelude::Address, BIP32Path, Seed};
use neon::prelude::*;

use std::{
  num::NonZeroU64,
  sync::{Arc, Mutex},
};

use super::{parse_address, Api, ClientTask};

pub struct ValueTransactionSender {
  client_id: String,
  seed: String,
  path: Arc<Mutex<Option<BIP32Path>>>,
  index: Arc<Mutex<Option<usize>>>,
  outputs: Arc<Mutex<Vec<(Address, NonZeroU64)>>>,
}

declare_types! {
    pub class JsValueTransactionSender for ValueTransactionSender {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(ValueTransactionSender {
                client_id,
                seed,
                path: Arc::new(Mutex::new(None)),
                index: Arc::new(Mutex::new(None)),
                outputs: Arc::new(Mutex::new(vec![]))
            })
        }

        method path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let path = BIP32Path::from_str(path.as_str()).expect("invalid bip32 path");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).path;
                let mut send_path = ref_.lock().unwrap();
                send_path.replace(path);
            }

            Ok(cx.this().upcast())
        }

        method index(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).index;
                let mut send_index = ref_.lock().unwrap();
                send_index.replace(index);
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

        method send(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));
                let client_task = ClientTask {
                    client_id: ref_.client_id.clone(),
                    api: Api::SendTransfer {
                        seed: Seed::from_ed25519_bytes(ref_.seed.as_bytes()).expect("invalid seed"),
                        path: (*ref_.path.lock().unwrap()).clone(),
                        index: *ref_.index.lock().unwrap(),
                        outputs: (*ref_.outputs.lock().unwrap()).clone(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
