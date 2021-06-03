// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use jni::{objects::JClass, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_org_iota_client_local_NativeAPI_verify_1link(_env: JNIEnv, _class: JClass) {
    // Were good
}
