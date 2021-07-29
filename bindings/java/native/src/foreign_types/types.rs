// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

foreign_typemap!(
    ($p:r_type) &[u8] => jbyteArray {
        let it: Vec<i8> = $p.iter().cloned().map(|x| x as i8).collect();
        $out = JavaByteArray::from_slice_to_raw(&it, env);
    };
);

foreign_typemap!(
    ($p:r_type) &[u8] <= JavaByteArray {
        let iter: Vec<u8> = $p.to_slice().iter().cloned().map(|x| x as u8).collect();
        $out = iter.as_slice();
    };
);

foreign_typemap!(
    ($p:r_type) Vec<u8> <= JavaByteArray {
        $out = $p.to_slice().iter().cloned().map(|x| x as u8).collect();
    };
);

foreign_typemap!(
    ($p:r_type) PathBuf => jstring {
        $out = from_std_string_jstring($p.as_path().display().to_string(), env);
    };
    ($p:f_type, option = "NoNullAnnotations", unique_prefix = "/*chrono*/")
        => "/*chrono*/java.nio.file.Path" "$out = java.nio.file.Paths.get($p);";
);

foreign_typemap!(
    ($p:r_type) &str => PathBuf {
        $out = PathBuf::from($p);
    };
);

foreign_typemap!(
    ($p:r_type) Result<bool> => bool {
        $out = match $p {
            Ok(x) => x,
            Err(err) => {
                let msg = err.to_string();
                let exception_class = match err {
                    _ => swig_jni_find_class!(WALLET_BASE_EXCEPTION, "org/iota/client/local/ClientException"),
                };
                jni_throw(env, exception_class, &msg);
                // We gotta return a value, but this never reaches the client side
                false
            }
        };
    };
);

// Yikes!
foreign_typemap!(
    ($p:r_type) Option<bool> => jshort {
        $out = match $p {
            Some(x) => if x { 1 } else { 0 },
            None => -1,
        };
    };
    ($p:f_type) => "java.util.Optional<java.lang.Boolean>" r#"
        $out;
        if ($p == -1 ) {
            $out = java.util.Optional.empty();
        } else {
            $out = java.util.Optional.of(java.lang.Boolean.valueOf($p == 1 ? true : false));
        }
"#;
);

foreign_typemap!(
    ($p:r_type) Option<u32> => jlong {
        $out = match $p {
            Some(x) => x.try_into().unwrap(),
            None => -1,
        };
    };
);

foreign_typemap!(
    ($p:r_type) Option<u8> => jint {
        $out = match $p {
            Some(x) => x.try_into().unwrap(),
            None => -1,
        };
    };
);

foreign_typemap!(
    ($p:r_type) &u64 => u64 {
        $out = *($p);
    };
);

foreign_typemap!(
    ($p:r_type) u128 => jobject {
        let data = $p.to_ne_bytes();
        let size = data.len();
        let arr: jbyteArray = (**env)->NewByteArray(env, size);
        (**env)->SetByteArrayRegion(env, arr, 0, size, data);
        
        let clazz: jclass = swig_jni_find_class!(U64_TO_BIGINT, "java.math.BigInteger");
        assert!(!jcls.is_null());
        constructor = (*env)->GetMethodID(env, clazz, "<init>", "([B)V");
        assert!(!constructor.is_null());
        object = (**env)->NewObject(env, clazz, constructor, arr);

        $out = object;
    };
);

foreign_typemap!(
    ($p:r_type) <T> Result<T> => swig_i_type!(T) {
        $out = match $p {
            Ok(x) => {
                swig_from_rust_to_i_type!(T, x, ret)
                ret
            }
            Err(err) => {
                let msg = err.to_string();
                let exception_class = match err {
                    _ => swig_jni_find_class!(CLIENT_BASE_EXCEPTION, "org/iota/client/local/ClientException"),
                };
                jni_throw(env, exception_class, &msg);
                return <swig_i_type!(T)>::jni_invalid_value();
            }
        };
    };
    ($p:f_type, unique_prefix="/*client::error::Result<swig_subst_type!(T)>*/") => "/*client::error::Result<swig_subst_type!(T)>*/swig_f_type!(T)"
        "swig_foreign_from_i_type!(T, $p)";
);

// Auto clone
foreign_typemap!(
    ($p:r_type) <T: SwigForeignClass + Clone> &T => swig_i_type!(T) {
        $out = swig_from_rust_to_i_type(T, $p.clone(), ret);
    };
);

// Duration
foreign_typemap!(
    ($p:r_type) Duration => jfloat {
        $out = $p.as_secs_f32();
    };
    ($p:f_type) => "java.time.Duration"
        r#"
        $out;
        java.time.Duration d = java.time.Duration.ofSeconds((int)$p);
        d.plusMillis((int)($p % 1 * 1000));
        $out = d;
"#;
);

foreign_typemap!(
    ($p:r_type) jfloat => Duration {
        $out = Duration::from_secs_f32($p);
    };
);

fn jstring_array_to_vec_of_string(
    env: *mut JNIEnv,
    arr: internal_aliases::JStringObjectsArray,
) -> Vec<String> {
    let length = unsafe { (**env).GetArrayLength.unwrap()(env, arr) };
    
    let len = <usize as ::std::convert::TryFrom<jsize>>::try_from(length)
        .expect("invalid jsize, in jsize => usize conversation");
    let mut result = Vec::with_capacity(len);
    for i in 0..length {
        let native: String = unsafe {
            let obj: jstring = (**env).GetObjectArrayElement.unwrap()(env, arr, i);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Failed to retrieve element {} from this `jobjectArray'", i);
            }
            let jstr = JavaString::new(env, obj);
            jstr.to_str().to_string()
        };
        result.push(native);
    }

    result
}

// String array to Vec<String>
foreign_typemap!(
    ($p:r_type) Vec<String> <= internal_aliases::JStringObjectsArray {
        $out = jstring_array_to_vec_of_string(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "java.lang.String []";
    ($p:f_type, option = "NullAnnotations")
                  <= "@NonNull java.lang.String []";
);

// Optional String array to Option<Vec<String>>
foreign_typemap!(
    ($p:r_type) Option<Vec<String>> <= internal_aliases::JStringObjectsArray {
        let opt;
        if $p.is_null() {
            opt = None;
        } else {
            opt = Some(jstring_array_to_vec_of_string(env, $p));
        }
        $out = opt;
    };
    
);

foreign_typemap!(
    ($p:r_type) Vec<Message> <= internal_aliases::JForeignObjectsArray<Message> {
        $out = jobject_array_to_vec_of_objects(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "Message[]";
    ($p:f_type, option = "NullAnnotations") <= "@NonNull Message[]";
);
foreign_typemap!(
    ($p:r_type) Vec<MessageId> <= internal_aliases::JForeignObjectsArray<MessageId> {
        $out = jobject_array_to_vec_of_objects(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "MessageId[]";
    ($p:f_type, option = "NullAnnotations") <= "@NonNull MessageId[]";
);
foreign_typemap!(
    ($p:r_type) Vec<Address> <= internal_aliases::JForeignObjectsArray<Address> {
        $out = jobject_array_to_vec_of_objects(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "Address[]";
    ($p:f_type, option = "NullAnnotations") <= "@NonNull Address[]";
);
foreign_typemap!(
    ($p:r_type) Vec<Topic> <= internal_aliases::JForeignObjectsArray<Topic> {
        $out = jobject_array_to_vec_of_objects(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "Topic[]";
    ($p:f_type, option = "NullAnnotations") <= "@NonNull Topic[]";
);
foreign_typemap!(
    ($p:r_type) Vec<UnlockBlock> <= internal_aliases::JForeignObjectsArray<UnlockBlock> {
        $out = jobject_array_to_vec_of_objects(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "UnlockBlock[]";
    ($p:f_type, option = "NullAnnotations") <= "@NonNull UnlockBlock[]";
);
foreign_typemap!(
    ($p:r_type) Vec<MigratedFundsEntry> <= internal_aliases::JForeignObjectsArray<MigratedFundsEntry> {
        $out = jobject_array_to_vec_of_objects(env, $p);
    };
    ($p:f_type, option = "NoNullAnnotations") <= "MigratedFundsEntry[]";
    ($p:f_type, option = "NullAnnotations") <= "@NonNull MigratedFundsEntry[]";
);