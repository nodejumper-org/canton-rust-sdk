//! Download the Daml packages a participant has vetted.
//!
//! Some packages ship as DAR files somebody publishes; others only ever exist
//! on a network. The token standard's V2 packages are the second kind at the
//! time of writing — live on DevNet, vetted by every participant there, and
//! absent from cn-quickstart, from the Splice repository's built artefacts, and
//! from any release asset. The participant is the only place to get them, and
//! it is also the most authoritative one: it is what the network agreed on.
//!
//! A package id is the hash of the package's own bytes, so the file names here
//! carry their own checksum. Nothing further needs pinning.
//!
//! ```sh
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TEST_TOKEN_URL=… CANTON_TEST_CLIENT_ID=… CANTON_TEST_CLIENT_SECRET=… \
//!   cargo run -p canton-admin --example fetch_packages -- <out-dir> [name-filter…]
//! ```
//!
//! With no filter it fetches everything the participant knows.

use canton_admin::{AdminClient, Config};
use canton_auth::{OidcConfig, TokenProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let out = args
        .next()
        .ok_or("usage: fetch_packages <out-dir> [name-filter…]")?;
    let filters: Vec<String> = args.collect();

    let endpoint = std::env::var("CANTON_TEST_ENDPOINT")
        .ok()
        .or_else(|| canton_core::localnet::grpc_endpoint(None))
        .ok_or("set CANTON_TEST_ENDPOINT")?;
    let config = Config::new(endpoint);
    let config = match (
        std::env::var("CANTON_TEST_TOKEN_URL"),
        std::env::var("CANTON_TEST_CLIENT_ID"),
        std::env::var("CANTON_TEST_CLIENT_SECRET"),
    ) {
        (Ok(url), Ok(id), Ok(secret)) => {
            config.with_oidc(TokenProvider::new(OidcConfig::new(url, id, secret)))
        }
        _ => match canton_core::localnet::token(None) {
            Some(token) => config.with_token(token),
            None => config,
        },
    };
    let admin = AdminClient::connect_lazy(config)?;

    std::fs::create_dir_all(&out)?;
    let ids = admin.list_packages().await?;
    println!("the participant knows {} packages", ids.len());

    let mut written = 0usize;
    let mut total = 0usize;
    for id in ids {
        let archive = admin.get_package(&id).await?;
        // The name is inside the payload, so it is read rather than guessed —
        // `list_packages` returns ids and nothing else.
        let package = canton_lf::decode_payload(&archive)?;
        let name = canton_lf::package_name(&package).unwrap_or("unnamed");
        let version = canton_lf::package_version(&package).unwrap_or("0.0.0");
        if !filters.is_empty() && !filters.iter().any(|f| name.contains(f.as_str())) {
            continue;
        }

        // `<name>-<version>-<id>.lfpayload`: the id is the content hash, so the
        // file name is its own checksum and two builds of one version never
        // collide. Not `.dalf` — a DAR entry is a whole `Archive` and this is
        // the payload alone, and naming it as though it were the other would
        // invite exactly the decode failure that named this file.
        let path = format!("{out}/{name}-{version}-{id}.lfpayload");
        std::fs::write(&path, &archive)?;
        println!(
            "{name:<45} {version:<8} {:>7} bytes  {}",
            archive.len(),
            &id[..16]
        );
        written += 1;
        total += archive.len();
    }
    println!("wrote {written} package(s), {} KB in total", total / 1024);
    Ok(())
}
