use bindgen;
use cbindgen;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=c_src/c-api.h");

    let crate_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined");

    bindgen::Builder::default()
        .header("c_src/c-api.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for c-api.h")
        .write_to_file(PathBuf::from(crate_dir.clone()).join("src/bindings.rs"))
        .expect("Couldn't write rust bindings for c-api.h!");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings for crate")
        .write_to_file(PathBuf::from(crate_dir).join("c_src/rust-c.h"));
}
