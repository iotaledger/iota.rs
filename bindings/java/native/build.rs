// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// CONFIGURATION
// add other android system headers to this list as necessary
static INCLUDE_SYS_H: [&str; 1] = ["jni.h"];

static RUST_SRC_DIR: &str = "src";
static ANDROID_ANNOTATION: &str = "androidx.annotation";
static PACKAGE_ID: &str = "org.iota.client";

static JNI_HEADERS_FILE: &str = "jni_c_headers.rs";

static ANDROID_TARGETS: [&str; 5] = [
    "aarch64-linux-android",
    "arm-linux-androideabi",
    "i686-linux-android",
    "x86_64-linux-android",
    "armv7-linux-androideabi",
];

#[path = "src/foreign_types/attributes.rs"]
mod attributes;

// =========================================================
// This script is portable; copy and paste it into your own
// build files at will.

use std::{
    env, fmt, fs,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    process::Stdio,
};

use bindgen::RustTarget;
use flapigen::{JavaConfig, JavaReachabilityFence, LanguageConfig};
use walkdir::WalkDir;

fn main() {
    // don't simplify this to if the target contains the substring "android" --
    // these lines also serve as a guard so only true android triples receive
    // JNI generation.
    let target = env::var("TARGET").unwrap();
    let include_dirs = if ANDROID_TARGETS.contains(&target.as_str()) {
        get_cc_system_include_dirs().expect("Can't get NDK's system include dirs")
    } else {
        let java_home = env::var("JAVA_HOME").expect("JAVA_HOME env variable not settted");

        let java_include_dir = Path::new(&java_home).join("include");

        let target = env::var("TARGET").expect("target env var not setted");
        let java_sys_include_dir = java_include_dir.join(if target.contains("windows") {
            "win32"
        } else if target.contains("darwin") {
            "darwin"
        } else {
            "linux"
        });

        [java_include_dir, java_sys_include_dir].to_vec()
    };

    gen_bindings(include_dirs, &target);
}

fn gen_bindings(include_dirs: Vec<PathBuf>, target: &str) {
    let include_headers: Vec<_> = INCLUDE_SYS_H
        .iter()
        .map(|h| search_file_in_directory(&include_dirs, h).unwrap_or_else(|_| panic!("Could not find header {h}")))
        .collect();

    let src_dir = Path::new(RUST_SRC_DIR);
    let out_dir = env::var("OUT_DIR").unwrap();

    gen_binding(
        target,
        &include_dirs,
        &include_headers,
        &Path::new(&out_dir).join(JNI_HEADERS_FILE),
    )
    .unwrap();

    // Find files ending in .rs.in and expand them with SWIG
    for _entry in WalkDir::new(RUST_SRC_DIR) {
        let entry = _entry.expect("Error walking sources.");
        if entry.path().is_dir() || !entry.path().to_string_lossy().ends_with(".rs.in") {
            continue;
        }

        println!("Found SWIG specification: {}", entry.path().display());
        let swigf = entry.path().strip_prefix("src").unwrap();

        flapigen_expand(target, src_dir, swigf, Path::new(&out_dir));

        write_include_file(src_dir, swigf).expect("Failed to write include file.");
    }

    // Hook up cargo reruns
    println!("cargo:rerun-if-changed={RUST_SRC_DIR}");
    for dir in &include_dirs {
        println!("cargo:rerun-if-changed={}", dir.display());
    }
    // if the generated files were deleted (e.g by gradle -q clean), regenerate them
    println!("cargo:rerun-if-changed={out_dir}");
}

fn get_cc_system_include_dirs() -> Result<Vec<PathBuf>, String> {
    let cc_build = cc::Build::new();

    let cc_process = cc_build
        .get_compiler()
        .to_command()
        .env("LANG", "C")
        .env("LC_MESSAGES", "C")
        .args(["-v", "-x", "c", "-E", "-"])
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|err| err.to_string())?;

    cc_process
        .stdin
        .ok_or_else(|| "can not get stdin of cc".to_string())?
        .write_all(b"\n")
        .map_err(|err| err.to_string())?;

    let mut cc_output = String::new();

    cc_process
        .stderr
        .ok_or_else(|| "can not get stderr of cc".to_string())?
        .read_to_string(&mut cc_output)
        .map_err(|err| err.to_string())?;

    const BEGIN_PAT: &str = "\n#include <...> search starts here:\n";
    const END_PAT: &str = "\nEnd of search list.\n";
    let start_includes = cc_output
        .find(BEGIN_PAT)
        .ok_or_else(|| format!("No '{BEGIN_PAT}' in output from C compiler"))?
        + BEGIN_PAT.len();
    let end_includes = cc_output[start_includes..]
        .find(END_PAT)
        .ok_or_else(|| format!("No '{END_PAT}' in output from C compiler"))?
        + start_includes;

    Ok(cc_output[start_includes..end_includes]
        .split('\n')
        .map(|s| PathBuf::from(s.trim().to_string()))
        .collect())
}

fn search_file_in_directory<P>(dirs: &[P], file: &str) -> Result<PathBuf, ()>
where
    P: AsRef<Path>,
{
    for dir in dirs {
        let file_path = dir.as_ref().join(file);
        if file_path.exists() && file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(())
}

fn gen_binding<P1, P2>(target: &str, include_dirs: &[P1], c_headers: &[P2], output_rust: &Path) -> Result<(), String>
where
    P1: AsRef<Path> + fmt::Debug,
    P2: AsRef<Path> + fmt::Debug,
{
    assert!(!c_headers.is_empty());
    let c_file_path = &c_headers[0];

    let mut bindings: bindgen::Builder = bindgen::builder().header(c_file_path.as_ref().to_str().unwrap());
    bindings = include_dirs.iter().fold(bindings, |acc, x| {
        acc.clang_arg("-I".to_string() + x.as_ref().to_str().unwrap())
    });
    println!("Generate binding for {c_headers:?}");
    bindings = bindings.rust_target(RustTarget::Stable_1_19);
    bindings = if target.contains("windows") {
        // see https://github.com/servo/rust-bindgen/issues/578
        bindings.trust_clang_mangling(false)
    } else {
        bindings
    };
    bindings = c_headers[1..]
        .iter()
        .fold(Ok(bindings), |acc: Result<bindgen::Builder, String>, header| {
            let c_file_path = header;
            let c_file_str = c_file_path
                .as_ref()
                .to_str()
                .ok_or_else(|| format!("Invalid unicode in path to {:?}", c_file_path.as_ref()))?;
            Ok(acc.unwrap().clang_arg("-include").clang_arg(c_file_str))
        })?;

    let generated_bindings = bindings
        //        .clang_arg(format!("-target {}", target))
        .generate()
        .map_err(|_| "Failed to generate bindings".to_string())?;
    generated_bindings
        .write_to_file(output_rust)
        .map_err(|err| err.to_string())?;

    Ok(())
}

fn flapigen_expand(target: &str, source_dir: &Path, file: &Path, out_dir: &Path) {
    let have_java_9 = fs::read_to_string(out_dir.join(JNI_HEADERS_FILE))
        .unwrap()
        .contains("JNI_VERSION_9");

    let mut java_cfg = JavaConfig::new(
        Path::new("src")
            .join("main")
            .join("java")
            .join(PACKAGE_ID.replace('.', "/")),
        PACKAGE_ID.to_string(),
    )
    .use_reachability_fence(if have_java_9 {
        JavaReachabilityFence::Std
    } else {
        JavaReachabilityFence::GenerateFence(8)
    });

    if ANDROID_TARGETS.contains(&target) {
        java_cfg = java_cfg.use_null_annotation_from_package(ANDROID_ANNOTATION.into());
    }

    let swig_gen = flapigen::Generator::new(LanguageConfig::JavaConfig(java_cfg))
        .rustfmt_bindings(false)
        .remove_not_generated_files_from_output_directory(false)
        .merge_type_map("chrono_support", include_str!("src/foreign_types/chrono_include.rs"))
        .merge_type_map("foreign_types", include_str!("src/foreign_types/types.rs"))
        .register_class_attribute_callback("PartialEq", attributes::class_partial_eq)
        .register_class_attribute_callback("Display", attributes::class_to_string);

    let out_file = out_dir.join(
        Path::new(file.parent().unwrap_or_else(|| Path::new(".")))
            .join(file.file_stem().expect("Got invalid file (no filename)")),
    );

    swig_gen.expand(PACKAGE_ID.as_ref(), source_dir.join(file), out_file);
}

fn write_include_file(source_dir: &Path, swig_file: &Path) -> std::io::Result<()> {
    let rs_rel_file = Path::new(swig_file.parent().unwrap_or_else(|| Path::new(".")))
        .join(swig_file.file_stem().expect("Got invalid file (no filename)"));
    let rs_path = source_dir.join(&rs_rel_file);

    if rs_path.exists() {
        println!("Not writing {} because it exists", rs_path.display());
        return Ok(());
    }

    let mut rs_file = File::create(rs_path)?;
    rs_file.write_all(
        format!(
            r#"// Automatically generated by Rust-SWIG
include!(concat!(env!("OUT_DIR"), "/{}"));"#,
            rs_rel_file.to_string_lossy()
        )
        .as_bytes(),
    )?;

    Ok(())
}
