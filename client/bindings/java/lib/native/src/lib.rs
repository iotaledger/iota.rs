// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Mutex;

use iota_client::message_interface::{ClientMessageHandler, Message};
use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

lazy_static! {
    static ref MESSAGE_HANDLER: Mutex<Option<ClientMessageHandler>> = Mutex::new(None);
}

#[no_mangle]
pub extern "system" fn Java_org_iota_apis_NativeApi_createMessageHandler(
    mut env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    config: JString,
) {
    let config: String = match env.get_string(&config) {
        Ok(jstring) => jstring.into(),
        Err(err) => {
            env.throw_new("java/lang/Exception", err.to_string()).unwrap();
            return;
        }
    };

    match MESSAGE_HANDLER.lock() {
        Ok(mut message_handler_store) => {
            // throw an exception if a message handler already exists
            if message_handler_store.is_some() {
                env.throw_new("java/lang/Exception", "a client instance was already created")
                    .unwrap();
                return;
            }

            match iota_client::message_interface::create_message_handler(Some(config)) {
                Ok(message_handler) => {
                    message_handler_store.replace(message_handler);
                }
                Err(err) => {
                    env.throw_new("java/lang/Exception", err.to_string()).unwrap();
                    // no return needed because no code has to be executed after
                }
            }
        }
        Err(err) => {
            env.throw_new("java/lang/Exception", err.to_string()).unwrap();
            // no return needed because no code has to be executed after
        }
    }
}

// This keeps rust from "mangling" the name and making it unique for this crate.
#[no_mangle]
pub extern "system" fn Java_org_iota_apis_NativeApi_sendCommand(
    mut env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    command: JString,
) -> jstring {
    if env.exception_check().unwrap() {
        return std::ptr::null_mut();
    }

    let command: String = env.get_string(&command).expect("Couldn't get java string!").into();

    let message = serde_json::from_str::<Message>(&command).unwrap();

    let guard = MESSAGE_HANDLER.lock().unwrap();

    let response = block_on(guard.as_ref().unwrap().send_message(message));

    let output = env
        .new_string(serde_json::to_string(&response).unwrap())
        .expect("Couldn't create java string!");

    output.into_raw()
}

// Destroy the required parts for messaging. Needs to call createMessageHandler again before resuming
#[no_mangle]
pub extern "system" fn Java_org_iota_apis_NativeApi_destroyHandle(_env: JNIEnv, _class: JClass) {
    (*MESSAGE_HANDLER.lock().unwrap()) = None;
}

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}
