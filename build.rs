extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Generate bindings for Porcupine parameters header file
    let bindings = bindgen::Builder::default()
        .header("assets/lib/pv_porcupine_params.h")
        .use_core()
        .ctypes_prefix("core::ffi")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link Porcupine static library
    println!("cargo:rustc-link-search=native=assets/lib");
    println!("cargo:rustc-link-lib=static=pv_porcupine");

    // Include `.ppn` file in build
    println!("cargo:rerun-if-changed=assets/model.ppn");
}
