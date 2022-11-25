// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Mutex;

use iota_client::message_interface::Message;
use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};
use once_cell::sync::OnceCell;
use tokio::{runtime::Runtime, sync::mpsc::unbounded_channel};

// This keeps rust from "mangling" the name and making it unique for this crate.
#[no_mangle]
pub extern "system" fn Java_org_iota_apis_BaseApi_callNativeLibrary(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    client_config: JString,
    client_command: JString,
) -> jstring {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let client_config: String = env.get_string(client_config).expect("Couldn't get java string!").into();

    let client_command: String = env
        .get_string(client_command)
        .expect("Couldn't get java string!")
        .into();

    let message_handler = crate::block_on(async {
        iota_client::message_interface::create_message_handler(Some(client_config.to_string()))
    })
    .unwrap();

    let message = serde_json::from_str::<Message>(&client_command).unwrap();

    let (sender, mut receiver) = unbounded_channel();
    crate::block_on(message_handler.handle(message, sender));
    let response = crate::block_on(receiver.recv()).unwrap();

    let output = env
        .new_string(serde_json::to_string(&response).unwrap())
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}
