//! Generates Rust gRPC client stubs from the vendored Canton Ledger API v2
//! `.proto` files (pinned to a Canton release under `proto/`).

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto");

    // Re-run if any proto is added/removed/edited (per-file tracking below only
    // covers files that already exist at generation time).
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SERVER");

    // Server stubs are generated only under the `server` feature, which is
    // dev-only (enabled by tests that need an in-process gRPC server). The
    // published SDK stays client-only.
    let build_server = std::env::var_os("CARGO_FEATURE_SERVER").is_some();

    // Vendor `protoc` so neither consumers nor docs.rs need a system install.
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build scripts run single-threaded before any codegen; setting an
    // env var here is the documented way to point prost/tonic at protoc.
    unsafe {
        std::env::set_var("PROTOC", &protoc);
    }

    // Compile every `.proto` under `proto/`.
    let mut protos = Vec::new();
    for entry in walkdir::WalkDir::new(&proto_root) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "proto")
        {
            protos.push(entry.path().to_path_buf());
        }
    }
    protos.sort();

    // Note: tonic-prost-build already maps `.google.protobuf` to `::prost_types`
    // by default, so we must not register that extern path again.
    tonic_prost_build::configure()
        .build_server(build_server)
        .build_client(true)
        .compile_protos(&protos, &[proto_root])?;

    Ok(())
}
