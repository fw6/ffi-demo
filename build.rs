#![allow(dead_code)]

use std::fs;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "wasm32" {
        #[cfg(feature = "snappy")]
        fail_on_empty_directory("deps/snappy");
        #[cfg(feature = "snappy")]
        build_snappy();

        #[cfg(feature = "jieba")]
        fail_on_empty_directory("deps/cppjieba-cabi");
        #[cfg(feature = "jieba")]
        fail_on_empty_directory("deps/cppjieba-cabi/cppjieba");
        #[cfg(feature = "jieba")]
        build_cjieba();

        #[cfg(feature = "bindgen")]
        build_bindgen();
    }
}

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

#[cfg(feature = "jieba")]
fn build_cjieba() {
    println!("[leveldb] Building");

    let mut config = cmake::Config::new(std::path::Path::new("deps").join("cppjieba-cabi"));

    config.build_target("cjieba_static");

    let dest_prefix = config.build();

    println!(
        "cargo:rustc-link-search=native={}/build",
        dest_prefix.display()
    );
    println!("cargo:rustc-link-lib=static=cjieba_static");

    link_cpp();
}

#[cfg(feature = "snappy")]
fn build_snappy() {
    println!("[snappy] Building");

    let dest_prefix = cmake::Config::new(std::path::Path::new("deps").join("snappy"))
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("SNAPPY_BUILD_TESTS", "OFF")
        .define("HAVE_LIBZ", "OFF")
        .build_target("snappy")
        .build();
    let build = dest_prefix.join("build");

    println!("cargo:rustc-link-search=native={}", build.display());
    println!("cargo:rustc-link-lib=static=snappy");

    link_cpp();
}

fn link_cpp() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") || target.contains("freebsd") {
        println!("cargo:rustc-link-lib=c++");
    } else if target.contains("gnu") || target.contains("netbsd") || target.contains("openbsd") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("musl") {
        // We want to link to libstdc++ *statically*. This requires that the user passes the right
        // search path to rustc via `-Lstatic=/path/to/libstdc++`.
        println!("cargo:rustc-link-lib=static=stdc++");
    }
}

#[cfg(feature = "bindgen")]
fn build_bindgen() {
    println!("cargo:rustc-link-lib=secp256k1");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
