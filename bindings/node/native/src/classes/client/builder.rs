// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, num::NonZeroU64, str::FromStr, time::Duration};

use iota::client::{Api, BrokerOptions, ClientBuilder};
use neon::prelude::*;

pub struct ClientBuilderWrapper {
    nodes: Vec<String>,
    broker_options: Option<BrokerOptions>,
    node_sync_interval: Option<NonZeroU64>,
    request_timeout: Option<Duration>,
    api_timeout: HashMap<Api, Duration>,
    local_pow: bool,
    node_sync_enabled: bool,
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
                local_pow: true,
                node_sync_enabled: true,
            })
        }

        method node(mut cx) {
            let node_url = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let nodes = &mut this.borrow_mut(&guard).nodes;
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
                let mut this = cx.this();
                let guard = cx.lock();
                let nodes = &mut this.borrow_mut(&guard).nodes;
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
                let mut this = cx.this();
                let guard = cx.lock();
                let broker_options = &mut this.borrow_mut(&guard).broker_options;
                broker_options.replace(options);
            }
            Ok(cx.this().upcast())
        }

        method nodeSyncInterval(mut cx) {
            let interval = cx.argument::<JsNumber>(0)?.value() as u64;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let interval_ref = &mut this.borrow_mut(&guard).node_sync_interval;
                interval_ref.replace(NonZeroU64::new(interval).expect("interval can't be zero"));
            }
            Ok(cx.this().upcast())
        }

        method disableNodeSync(mut cx) {
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let node_sync_enabled = &mut this.borrow_mut(&guard).node_sync_enabled;
                *node_sync_enabled = false;
            }
            Ok(cx.this().upcast())
        }

        method requestTimeout(mut cx) {
            let timeout = cx.argument::<JsNumber>(0)?.value() as u64;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let request_timeout = &mut this.borrow_mut(&guard).request_timeout;
                request_timeout.replace(Duration::from_millis(timeout));
            }
            Ok(cx.this().upcast())
        }

        method apiTimeout(mut cx) {
            let api = cx.argument::<JsString>(0)?.value();
            let api = Api::from_str(&api).expect("unknown api kind");
            let timeout = cx.argument::<JsNumber>(1)?.value() as u64;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let api_timeout_map = &mut this.borrow_mut(&guard).api_timeout;
                api_timeout_map.insert(api, Duration::from_millis(timeout));
            }
            Ok(cx.this().upcast())
        }

        method localPow(mut cx) {
            let local_pow = cx.argument::<JsBoolean>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let local_pow_ref = &mut this.borrow_mut(&guard).local_pow;
                *local_pow_ref = local_pow;
            }
            Ok(cx.this().upcast())
        }

        method build(mut cx) {
            let client = {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &*this.borrow(&guard);
                let mut builder = ClientBuilder::new().local_pow(ref_.local_pow);

                for node in &ref_.nodes {
                    builder = builder.node(node.as_str()).unwrap_or_else(|_| panic!("invalid node url: {}", node));
                }
                if let Some(broker_options) = &ref_.broker_options {
                    builder = builder.broker_options(broker_options.clone());
                }
                if ref_.node_sync_enabled {
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
