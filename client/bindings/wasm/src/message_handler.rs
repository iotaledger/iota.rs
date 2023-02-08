// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use iota_client::message_interface::{create_message_handler, ClientMessageHandler, Message, Response};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::future_to_promise;

/// The Client message handler.
#[wasm_bindgen(js_name = MessageHandler)]
pub struct WasmMessageHandler {
    handler: Rc<ClientMessageHandler>,
}

/// Creates a message handler with the given client options.
#[wasm_bindgen(js_name = messageHandlerNew)]
#[allow(non_snake_case)]
pub fn message_handler_new(clientOptions: Option<String>) -> Result<WasmMessageHandler, JsValue> {
    let client_message_handler: ClientMessageHandler = create_message_handler(clientOptions)
        .map_err(|err| js_sys::Error::new(&format!("Client MessageHandler constructor failed: {err}")))?;

    Ok(WasmMessageHandler {
        handler: Rc::new(client_message_handler),
    })
}

/// Handles a message, returns the response as a JSON-encoded string.
///
/// Returns an error if the response itself is an error or panic.
#[wasm_bindgen(js_name = sendMessageAsync)]
#[allow(non_snake_case)]
pub fn send_message_async(message: String, messageHandler: &WasmMessageHandler) -> Result<PromiseString, JsValue> {
    let message_handler: Rc<ClientMessageHandler> = Rc::clone(&messageHandler.handler);

    let promise: js_sys::Promise = future_to_promise(async move {
        let response: Response = send_message_inner(message_handler.as_ref(), message).await?;

        let ser =
            JsValue::from(serde_json::to_string(&response).map_err(|err| {
                JsValue::from_str(&format!("Client MessageHandler failed to serialize response: {err}"))
            })?);
        match response {
            Response::Error(_) | Response::Panic(_) => Err(ser),
            _ => Ok(ser),
        }
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into())
}

/// Handles a JSON-encoded message.
///
/// Returns `Response::Error` on deserialization errors.
async fn send_message_inner(handler: &ClientMessageHandler, serialized_message: String) -> Result<Response, JsValue> {
    let message: Message = match serde_json::from_str(&serialized_message) {
        Ok(msg) => msg,
        Err(err) => return Ok(Response::Error(iota_client::Error::Json(err))),
    };

    let response = handler.send_message(message).await;

    Ok(response)
}

/// MQTT is not supported for WebAssembly bindings.
///
/// Throws an error if called, only included for compatibility
/// with the Node.js bindings TypeScript definitions.
#[wasm_bindgen]
pub fn listen(_topics: ArrayString, _callback: &js_sys::Function) -> Result<(), JsValue> {
    let js_error = js_sys::Error::new("Client MQTT not supported for WebAssembly");

    Err(JsValue::from(js_error))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type ArrayString;

    #[wasm_bindgen(typescript_type = "Promise<string>")]
    pub type PromiseString;
}
