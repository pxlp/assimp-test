extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let module_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", module_path);

    println!("cargo:rustc-link-lib=static=assimp");
    //
    // // Link to libstdc++ on GNU
    // if target.contains("gnu") {
    //     println!("cargo:rustc-link-lib=stdc++");
    // }

    println!("cargo:rerun-if-changed=build.rs");
}
