// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//ANCHOR: foreign_typemap_chrono_example
foreign_typemap!(
    ($p:r_type) DateTime<Utc> => jlong {
        $out = $p.timestamp_millis();
    };
    ($p:f_type, option = "NoNullAnnotations", unique_prefix = "/*chrono*/")
        => "/*chrono*/java.util.Date" "$out = new java.util.Date($p);";
    ($p:f_type, option = "NullAnnotations", unique_prefix = "/*chrono*/")
        => "/*chrono*/@NonNull java.util.Date" "$out = new java.util.Date($p);";
);

foreign_typemap!(
    ($p:r_type) DateTime<Local> <= jlong {
        let d = std::time::UNIX_EPOCH + Duration::from_millis($p as u64);
        $out = DateTime::<Local>::from(d);
    };
);

foreign_typemap!(
    ($p:r_type) DateTime<Local> => jlong {
        $out = $p.timestamp_millis();
    };
    ($p:f_type) => "java.util.Calendar"
        r#"
        $out;

        java.util.Calendar theCalendar = java.util.Calendar.getInstance();
        theCalendar.setTime(new java.util.Date($p));

        $out = theCalendar;
"#;
);

//ANCHOR_END: foreign_typemap_chrono_example
foreign_typemap!(
    ($p:r_type) Option<DateTime<Local>> => jlong {
        $out = match $p {
            Some(x) => x.timestamp_millis(),
            None => -1,
        };
    };
    ($p:f_type) => "java.util.Optional<java.util.Calendar>"
        r#"
        $out;
        if ($p == -1 ) {
            $out = java.util.Optional.empty();
        } else {
            java.util.Calendar theCalendar = java.util.Calendar.getInstance();
            theCalendar.setTime(new java.util.Date($p));

            $out = java.util.Optional.of(theCalendar);
        }
"#;
);

foreign_typemap!(
    ($p:r_type) Option<DateTime<Utc>> => internal_aliases::JOptionalLong {
        let tmp: Option<i64> = $p.map(|x| x.timestamp_millis());
        $out = to_java_util_optional_long(env, tmp);
    };
    ($p:f_type, unique_prefix = "/*chrono*/") => "/*chrono*/java.util.Optional<java.util.Date>"
        r#"
        $out;
        if ($p.isPresent()) {
            $out = java.util.Optional.of(new java.util.Date($p.getAsLong()));
        } else {
            $out = java.util.Optional.empty();
        }
"#;
);
