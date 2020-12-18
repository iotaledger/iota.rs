// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::HashMap,
    num::NonZeroU64,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use iota::client::{Api, BrokerOptions, ClientBuilder};
use neon::prelude::*;

pub struct ClientBuilderWrapper {
    nodes: Arc<Mutex<Vec<String>>>,
    broker_options: Arc<Mutex<Option<BrokerOptions>>>,
    node_sync_interval: Arc<Mutex<Option<NonZeroU64>>>,
    request_timeout: Arc<Mutex<Option<Duration>>>,
    api_timeout: Arc<Mutex<HashMap<Api, Duration>>>,
    local_pow: Arc<AtomicBool>,
    node_sync_enabled: Arc<AtomicBool>,
}

declare_types! {
    pub class JsClientBuilder for ClientBuilderWrapper {
        init(_) {
            Ok(ClientBuilderWrapper {
                nodes: Default::default(),
                broker_options: Default::default(),
                node_sync_interval: Default::default(),
                request_timeout: Default::default(),
                api_timeout: Default::default(),
                local_pow: Arc::new(AtomicBool::new(true)),
                node_sync_enabled: Arc::new(AtomicBool::new(true)),
            })
        }

        method node(mut cx) {
            let node_url = cx.argument::<JsString>(0)?.value();
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).nodes;
                let mut nodes = ref_.lock().unwrap();
                nodes.push(node_url);
            }
            Ok(cx.this().upcast())
        }

        method nodes(mut cx) {
            let js_node_urls = cx.argument::<JsArray>(0)?;
            let js_node_urls: Vec<Handle<JsValue>> = js_node_urls.to_vec(&mut cx)?;
            let mut node_urls = vec![];
            for js_node_url in js_node_urls {
                let node_url: Handle<JsString> = js_node_url.downcast_or_throw(&mut cx)?;
                node_urls.push(node_url.value());
            }
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).nodes;
                let mut nodes = ref_.lock().unwrap();
                for node_url in node_urls {
                    nodes.push(node_url);
                }
            }
            Ok(cx.this().upcast())
        }

        method brokerOptions(mut cx) {
            let options = cx.argument::<JsString>(0)?.value();
            let options: BrokerOptions = serde_json::from_str(&options).expect("invalid broker options JSON");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).broker_options;
                let mut broker_options_ref = ref_.lock().unwrap();
                broker_options_ref.replace(options);
            }
            Ok(cx.this().upcast())
        }

        method nodeSyncInterval(mut cx) {
            let interval = cx.argument::<JsNumber>(0)?.value() as u64;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).node_sync_interval;
                let mut interval_ref = ref_.lock().unwrap();
                interval_ref.replace(NonZeroU64::new(interval).expect("interval can't be zero"));
            }
            Ok(cx.this().upcast())
        }

        method disableNodeSync(mut cx) {
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).node_sync_enabled;
                ref_.swap(false, Ordering::Relaxed);
            }
            Ok(cx.this().upcast())
        }

        method requestTimeout(mut cx) {
            let timeout = cx.argument::<JsNumber>(0)?.value() as u64;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).request_timeout;
                let mut request_timeout_ref = ref_.lock().unwrap();
                request_timeout_ref.replace(Duration::from_millis(timeout));
            }
            Ok(cx.this().upcast())
        }

        method apiTimeout(mut cx) {
            let api = cx.argument::<JsString>(0)?.value();
            let api = Api::from_str(&api).expect("unknown api kind");
            let timeout = cx.argument::<JsNumber>(1)?.value() as u64;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).api_timeout;
                let mut api_timeout_map = ref_.lock().unwrap();
                api_timeout_map.insert(api, Duration::from_millis(timeout));
            }
            Ok(cx.this().upcast())
        }

        method localPow(mut cx) {
            let local_pow = cx.argument::<JsBoolean>(0)?.value();
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).local_pow;
                ref_.swap(local_pow, Ordering::Relaxed);
            }
            Ok(cx.this().upcast())
        }

        method build(mut cx) {
            let client = {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &*this.borrow(&guard);
                let mut builder = ClientBuilder::new().local_pow(ref_.local_pow.load(Ordering::Relaxed));

                for node in &*ref_.nodes.lock().unwrap() {
                    builder = builder.node(node.as_str()).unwrap_or_else(|_| panic!("invalid node url: {}", node));
                }
                if let Some(broker_options) = &*ref_.broker_options.lock().unwrap() {
                    builder = builder.broker_options(broker_options.clone());
                }
                if ref_.node_sync_enabled.load(Ordering::Relaxed) {
                    builder = builder.disable_node_sync();
                }

                builder.build().expect("failed to build client instance")
            };
            let id = crate::store_client(client);
            let id = cx.string(id);
            Ok(super::JsClient::new(&mut cx, vec![id])?.upcast())
        }
    }
}
