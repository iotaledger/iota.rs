use iota::ClientBuilder;
use neon::prelude::*;

pub struct ClientWrapper(String);

impl Drop for ClientWrapper {
    fn drop(&mut self) {
        crate::remove_client(self.0.clone());
    }
}

declare_types! {
    pub class JsClient for ClientWrapper {
        init(mut cx) {
            let mut client_builder = ClientBuilder::new();
            let node_arg = cx.argument::<JsValue>(0)?;

            if let Ok(node) = node_arg.downcast::<JsString>() {
                client_builder = client_builder.node(&node.value()).expect(&format!("invalid node url: `{}`", node.value()));
            } else if let Ok(node_js_array) = node_arg.downcast::<JsArray>() {
                 let nodes: Vec<Handle<JsValue>> = node_js_array.to_vec(&mut cx)?;
                 for node in nodes {
                      let node: Handle<JsString> = node.downcast_or_throw(&mut cx)?;
                      client_builder = client_builder.node(&node.value()).expect(&format!("invalid node url: `{}`", node.value()));
                 }
            } else {
                return cx.throw_error("invalid node type, expected string or array of strings");
            }

            let client = client_builder.build().expect("failed to build client instance");
            let id = crate::store_client(client);
            Ok(ClientWrapper(id))
        }

        method subscriber(mut cx) {
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);
            Ok(crate::JsTopicSubscriber::new(&mut cx, vec![client_id])?.upcast())
        }
    }
}
