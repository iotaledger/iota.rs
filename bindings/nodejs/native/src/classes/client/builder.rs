// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::unnecessary_wraps)]
use std::{collections::HashMap, num::NonZeroU64, str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};

use iota_client::{Api, BrokerOptions, ClientBuilder};
use neon::prelude::*;

pub struct ClientBuilderWrapper {
    nodes: Vec<String>,
    auth: Option<(String, NodeAuthOptions)>,
    primary_node: Option<(String, Option<NodeAuthOptions>)>,
    primary_pow_node: Option<(String, Option<NodeAuthOptions>)>,
    node_pool_urls: Vec<String>,
    quorum: Option<bool>,
    quorum_size: Option<usize>,
    quorum_threshold: Option<usize>,
    network: Option<String>,
    broker_options: Option<BrokerOptions>,
    node_sync_interval: Option<NonZeroU64>,
    request_timeout: Option<Duration>,
    api_timeout: HashMap<Api, Duration>,
    local_pow: bool,
    tips_interval: u64,
    node_sync_disabled: bool,
}

#[derive(Serialize, Deserialize)]
struct NodeAuthOptions {
    jwt: Option<String>,
    #[serde(rename = "basicAuthName")]
    basic_auth_name: Option<String>,
    #[serde(rename = "basicAuthPassword")]
    basic_auth_password: Option<String>,
}

declare_types! {
    pub class JsClientBuilder for ClientBuilderWrapper {
        init(_) {
            Ok(ClientBuilderWrapper {
                nodes: Default::default(),
                auth: Default::default(),
                primary_node: Default::default(),
                primary_pow_node: Default::default(),
                node_pool_urls: Default::default(),
                quorum: Default::default(),
                quorum_size: Default::default(),
                quorum_threshold: Default::default(),
                network: Default::default(),
                broker_options: Default::default(),
                node_sync_interval: Default::default(),
                request_timeout: Default::default(),
                api_timeout: Default::default(),
                local_pow: true,
                tips_interval: 15,
                node_sync_disabled: false,
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

        method nodeAuth(mut cx) {
            let node_url = cx.argument::<JsString>(0)?.value();
            let auth_options = cx.argument::<JsString>(1)?.value();
            let options: NodeAuthOptions = serde_json::from_str(&auth_options).expect("invalid node auth options JSON");
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let auth = &mut this.borrow_mut(&guard).auth;
                auth.replace((node_url, options));
            }
            Ok(cx.this().upcast())
        }

        method primaryNode(mut cx) {
            let node_url = cx.argument::<JsString>(0)?.value();
            let auth_options: Option<NodeAuthOptions> = match cx.argument_opt(1) {
                Some(arg) => {
                    let auth_options = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    let options: NodeAuthOptions = serde_json::from_str(&auth_options).expect("invalid node auth options JSON");
                    Some(options)
                },
                None => None,
            };
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let primary_node = &mut this.borrow_mut(&guard).primary_node;
                primary_node.replace((node_url, auth_options));
            }
            Ok(cx.this().upcast())
        }

        method primaryPowNode(mut cx) {
            let node_url = cx.argument::<JsString>(0)?.value();
            let auth_options: Option<NodeAuthOptions> = match cx.argument_opt(1) {
                Some(arg) => {
                    let auth_options = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    let options: NodeAuthOptions = serde_json::from_str(&auth_options).expect("invalid node auth options JSON");
                    Some(options)
                },
                None => None,
            };
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let primary_pow_node = &mut this.borrow_mut(&guard).primary_pow_node;
                primary_pow_node.replace((node_url, auth_options));
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

        method nodePoolUrls(mut cx) {
            let js_node_urls = cx.argument::<JsArray>(0)?;
            let js_node_urls: Vec<Handle<JsValue>> = js_node_urls.to_vec(&mut cx)?;

            let mut node_pool_urls = vec![];
            for js_node_url in js_node_urls {
                let node_url: Handle<JsString> = js_node_url.downcast_or_throw(&mut cx)?;
                node_pool_urls.push(node_url.value());
            }

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let pool_urls = &mut this.borrow_mut(&guard).node_pool_urls;
                for node_pool_url in node_pool_urls {
                    pool_urls.push(node_pool_url);
                }
            }

            Ok(cx.this().upcast())
        }

        method quorum(mut cx) {
            let enabled = cx.argument::<JsBoolean>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let quorum = &mut this.borrow_mut(&guard).quorum;
                quorum.replace(enabled);
            }
            Ok(cx.this().upcast())
        }

        method quorumSize(mut cx) {
            let size = cx.argument::<JsNumber>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let quorum_size = &mut this.borrow_mut(&guard).quorum_size;
                quorum_size.replace(size as usize);
            }
            Ok(cx.this().upcast())
        }

        method quorumThreshold(mut cx) {
            let threshold = cx.argument::<JsNumber>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let quorum_threshold = &mut this.borrow_mut(&guard).quorum_threshold;
                quorum_threshold.replace(threshold as usize);
            }
            Ok(cx.this().upcast())
        }

        method network(mut cx) {
            let network_name = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let network = &mut this.borrow_mut(&guard).network;
                network.replace(network_name);
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
                let node_sync_disabled = &mut this.borrow_mut(&guard).node_sync_disabled;
                *node_sync_disabled = true;
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

        method tipsInterval(mut cx) {
            let tips_interval = cx.argument::<JsNumber>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let tips_interval_ref = &mut this.borrow_mut(&guard).tips_interval;
                *tips_interval_ref = tips_interval as u64;
            }
            Ok(cx.this().upcast())
        }

        method build(mut cx) {
            let client = {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &*this.borrow(&guard);
                let mut builder = ClientBuilder::new().with_local_pow(ref_.local_pow).with_tips_interval(ref_.tips_interval);

                for node in &ref_.nodes {
                    builder = builder.with_node(node.as_str()).unwrap_or_else(|_| panic!("invalid node url: {}", node));
                }
                if let Some((node, auth_options)) = &ref_.auth{
                    if let (Some(name), Some(password)) = (auth_options.basic_auth_name.as_ref(), auth_options.basic_auth_password.as_ref()){
                        builder = builder.with_node_auth(node, auth_options.jwt.clone(), Some((name, password))).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                    } else{
                        builder = builder.with_node_auth(node, auth_options.jwt.clone(), None).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                    }
                }
                if let Some((node, auth_options)) = &ref_.primary_node{
                    match auth_options{
                        Some(auth_options) => {
                            if let (Some(name), Some(password)) = (auth_options.basic_auth_name.as_ref(), auth_options.basic_auth_password.as_ref()){
                                builder = builder.with_primary_node(node, auth_options.jwt.clone(), Some((&name, &password))).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                            }else{
                                builder = builder.with_primary_node(node, auth_options.jwt.clone(), None).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                            }
                        },
                        None => {
                            builder = builder.with_primary_node(node, None, None).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                        }
                    }
                }
                if let Some((node, auth_options)) = &ref_.primary_pow_node{
                    match auth_options{
                        Some(auth_options) => {
                            if let (Some(name), Some(password)) = (auth_options.basic_auth_name.as_ref(), auth_options.basic_auth_password.as_ref()){
                                builder = builder.with_primary_pow_node(node, auth_options.jwt.clone(), Some((&name, &password))).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                            }else{
                                builder = builder.with_primary_pow_node(node, auth_options.jwt.clone(), None).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                            }
                        },
                        None => {
                            builder = builder.with_primary_pow_node(node, None, None).unwrap_or_else(|_| panic!("invalid node url: {} or auth parameters", node));
                        }
                    }
                }
                if !&ref_.node_pool_urls.is_empty() {
                    builder = crate::block_on(builder.with_node_pool_urls(&ref_.node_pool_urls)).expect("Problem with node pool url");
                }
                if let Some(enabled) = &ref_.quorum {
                    builder = builder.with_quorum(*enabled);
                }
                if let Some(size) = &ref_.quorum_size {
                    builder = builder.with_quorum_size(*size);
                }
                if let Some(threshold) = &ref_.quorum_threshold {
                    builder = builder.with_quorum_threshold(*threshold);
                }
                if let Some(network_name) = &ref_.network {
                    builder = builder.with_network(network_name);
                }
                if let Some(broker_options) = &ref_.broker_options {
                    builder = builder.with_mqtt_broker_options(broker_options.clone());
                }
                if ref_.node_sync_disabled {
                    builder = builder.with_node_sync_disabled();
                }

                crate::block_on(builder.finish()).expect("failed to build client instance")
            };
            let id = crate::store_client(client);
            let id = cx.string(id);
            Ok(super::JsClient::new(&mut cx, vec![id])?.upcast())
        }
    }
}
