//! Generate Rust types from the vendored Daml-LF archive `.proto` files
//! (`daml_lf.proto` + `daml_lf2.proto`, under `proto/`, pinned to a Daml 3.3
//! snapshot). LF-LF messages only — no gRPC services — so plain `prost-build`.

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto");
    println!("cargo:rerun-if-changed=proto");

    // Vendor `protoc` so no system install is needed.
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build scripts run single-threaded before codegen; this is the
    // documented way to point prost at protoc.
    unsafe {
        std::env::set_var("PROTOC", &protoc);
    }

    let protos = [
        proto_root.join("com/digitalasset/daml/lf/archive/daml_lf.proto"),
        proto_root.join("com/digitalasset/daml/lf/archive/daml_lf2.proto"),
    ];

    // `include_file` wires the two proto packages (`daml_lf_dev`, `daml_lf_2`)
    // into one module tree with correct cross-package paths.
    prost_build::Config::new()
        .include_file("_daml_lf.rs")
        .compile_protos(&protos, &[proto_root])?;
    Ok(())
}
