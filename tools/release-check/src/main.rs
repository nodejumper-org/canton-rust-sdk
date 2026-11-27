//! The 0.3.3 release, exercised as an application would use it: every area the
//! release changed, through the public API, against the Canton Network DevNet
//! where a participant is what answers. Prints one line per check and exits
//! non-zero if any failed. Reads endpoints and credentials from the
//! environment and prints none of them.

use std::time::{Duration, Instant};

use canton::admin::{AdminClient, ExpectedHosting, ExternalPartyTopology};
use canton::auth::{OidcConfig, TokenProvider};
use canton::ledger::hashing::hash_prepared_transaction;
use canton::ledger::proto::interactive as ipb;
use canton::ledger::proto::interactive::transaction::v1 as node;
use canton::ledger::{CantonClient, Error, JsonClient, Prepare, identifier, record, value};
use canton::signer::{Ed25519Key, KeyFormat, KeySpec, PublicKey, Signer as _};
use prost::Message as _;

type Outcome = Result<String, String>;

struct Report {
    failed: u32,
    passed: u32,
}

impl Report {
    fn record(&mut self, area: &str, name: &str, outcome: Outcome) {
        match outcome {
            Ok(detail) => {
                self.passed += 1;
                println!("PASS  [{area}] {name}: {detail}");
            }
            Err(detail) => {
                self.failed += 1;
                println!("FAIL  [{area}] {name}: {detail}");
            }
        }
    }
}

fn env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{name} is not set"))
}

fn oidc() -> Result<TokenProvider, String> {
    Ok(TokenProvider::new(OidcConfig::new(
        env("CANTON_TEST_TOKEN_URL")?,
        env("CANTON_TEST_ADMIN_CLIENT_ID")?,
        env("CANTON_TEST_ADMIN_CLIENT_SECRET")?,
    )))
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(ToString::to_string).collect()
}

fn wrap(id: &str, node_type: node::node::NodeType) -> ipb::daml_transaction::Node {
    ipb::daml_transaction::Node {
        node_id: id.to_string(),
        versioned_node: Some(ipb::daml_transaction::node::VersionedNode::V1(node::Node {
            node_type: Some(node_type),
        })),
    }
}

fn fetch(contract_id: &str) -> node::node::NodeType {
    node::node::NodeType::Fetch(node::Fetch {
        lf_version: "2.1".to_string(),
        contract_id: contract_id.to_string(),
        package_name: "package".to_string(),
        template_id: Some(identifier("package", "Module", "Entity")),
        signatories: strings(&["alice"]),
        stakeholders: strings(&["alice"]),
        acting_parties: strings(&["alice"]),
        ..Default::default()
    })
}

fn rollback(children: &[&str]) -> node::node::NodeType {
    node::node::NodeType::Rollback(node::Rollback {
        children: strings(children),
    })
}

fn metadata(act_as: &[&str]) -> ipb::Metadata {
    ipb::Metadata {
        submitter_info: Some(ipb::metadata::SubmitterInfo {
            act_as: strings(act_as),
            command_id: "command-1".to_string(),
        }),
        synchronizer_id: "synchronizer::1220aa".to_string(),
        transaction_uuid: "4c6471d3-4e09-49dd-addf-6cd90e19c583".to_string(),
        preparation_time: 1_700_000_000_000_000,
        ..Default::default()
    }
}

fn prepared(
    nodes: Vec<ipb::daml_transaction::Node>,
    roots: &[&str],
    metadata: ipb::Metadata,
) -> ipb::PreparedTransaction {
    ipb::PreparedTransaction {
        transaction: Some(ipb::DamlTransaction {
            version: "2.1".to_string(),
            roots: strings(roots),
            nodes,
            node_seeds: Vec::new(),
        }),
        metadata: Some(metadata),
    }
}

const CID: &str = "0007e7b5534931dfca8e1b485c105bae4e10808bd13ddc8e897f258015f9d921c5";
const V2: ipb::HashingSchemeVersion = ipb::HashingSchemeVersion::V2;
const V3: ipb::HashingSchemeVersion = ipb::HashingSchemeVersion::V3;

/// #18, #19, #23, #26: the hasher, on inputs a participant could send.
fn hashing(report: &mut Report) {
    // #18: a contract id that is not lower-case hex is an error, not a panic.
    let outcome = (|| {
        for id in ["aéb", "0é0", "+f", "AB", "abc"] {
            let tx = prepared(vec![wrap("0", fetch(id))], &["0"], metadata(&["alice"]));
            for scheme in [V2, V3] {
                match std::panic::catch_unwind(|| hash_prepared_transaction(&tx, scheme)) {
                    Ok(Err(_)) => {}
                    Ok(Ok(_)) => return Err(format!("contract id {id:?} hashed under {scheme:?}")),
                    Err(_) => return Err(format!("contract id {id:?} panicked the hasher")),
                }
            }
        }
        let good = prepared(vec![wrap("0", fetch(CID))], &["0"], metadata(&["alice"]));
        hash_prepared_transaction(&good, V2).map_err(|e| format!("a well-formed id: {e}"))?;
        Ok(
            "5 malformed contract ids refused under V2 and V3, a well-formed one hashed"
                .to_string(),
        )
    })();
    report.record("hashing", "#18 contract ids are decoded as bytes", outcome);

    // #19: 60 levels, each listing the next twice, in milliseconds; cycles refused.
    let outcome = (|| {
        let mut nodes = vec![wrap("0", fetch(CID))];
        for level in 1..=60 {
            let child = (level - 1).to_string();
            nodes.push(wrap(&level.to_string(), rollback(&[&child, &child])));
        }
        let tx = prepared(nodes, &["60"], metadata(&["alice"]));
        let started = Instant::now();
        hash_prepared_transaction(&tx, V2).map_err(|e| e.to_string())?;
        hash_prepared_transaction(&tx, V3).map_err(|e| e.to_string())?;
        let took = started.elapsed();
        if took > Duration::from_secs(2) {
            return Err(format!("60 levels took {took:?}"));
        }
        let own_child = prepared(
            vec![wrap("0", rollback(&["0"]))],
            &["0"],
            metadata(&["alice"]),
        );
        let two_cycle = prepared(
            vec![wrap("0", rollback(&["1"])), wrap("1", rollback(&["0"]))],
            &["0"],
            metadata(&["alice"]),
        );
        for (what, tx) in [
            ("a node listing itself", own_child),
            ("a two-node cycle", two_cycle),
        ] {
            if hash_prepared_transaction(&tx, V2).is_ok() {
                return Err(format!("{what} hashed"));
            }
        }
        Ok(format!(
            "60 doubled levels hashed twice in {took:?}; both cycles refused"
        ))
    })();
    report.record("hashing", "#19 a shared node is hashed once", outcome);

    // #23: the order the acting parties arrive in is not part of the hash.
    let outcome = (|| {
        let tx = |act_as: &[&str]| prepared(vec![wrap("0", fetch(CID))], &["0"], metadata(act_as));
        let sorted =
            hash_prepared_transaction(&tx(&["alice", "bob"]), V2).map_err(|e| e.to_string())?;
        let reversed =
            hash_prepared_transaction(&tx(&["bob", "alice"]), V2).map_err(|e| e.to_string())?;
        let repeated = hash_prepared_transaction(&tx(&["alice", "bob", "alice"]), V2)
            .map_err(|e| e.to_string())?;
        if sorted != reversed || sorted != repeated {
            return Err(
                "the metadata hash depends on the order or repetition of act_as".to_string(),
            );
        }
        Ok("act_as reversed and repeated give the same hash".to_string())
    })();
    report.record(
        "hashing",
        "#23 metadata is hashed as Canton orders it",
        outcome,
    );

    // #26: a max record time is refused under V2 and signed under V3.
    let outcome = (|| {
        let tx = |deadline: Option<u64>| {
            let mut metadata = metadata(&["alice"]);
            metadata.max_record_time = deadline;
            prepared(vec![wrap("0", fetch(CID))], &["0"], metadata)
        };
        if hash_prepared_transaction(&tx(Some(1_800_000_000_000_000)), V2).is_ok() {
            return Err("V2 hashed a transaction with a max record time".to_string());
        }
        hash_prepared_transaction(&tx(None), V2).map_err(|e| e.to_string())?;
        let as_prepared = hash_prepared_transaction(&tx(Some(1_800_000_000_000_000)), V3)
            .map_err(|e| e.to_string())?;
        let moved = hash_prepared_transaction(&tx(Some(1_900_000_000_000_000)), V3)
            .map_err(|e| e.to_string())?;
        let removed = hash_prepared_transaction(&tx(None), V3).map_err(|e| e.to_string())?;
        if as_prepared == moved || as_prepared == removed || moved == removed {
            return Err("V3 does not tell the deadlines apart".to_string());
        }
        Ok(
            "refused under V2; as prepared, moved and removed are three hashes under V3"
                .to_string(),
        )
    })();
    report.record(
        "hashing",
        "#26 a max record time is covered or refused",
        outcome,
    );
}

/// #20: both rustls providers compiled in, none installed.
async fn tls_providers(report: &mut Report) {
    let unusable = rustls::crypto::CryptoProvider::get_default().is_none()
        && std::panic::catch_unwind(|| {
            let _ = rustls::ClientConfig::builder();
        })
        .is_err();
    report.record(
        "tls",
        "#20 precondition: rustls cannot pick a provider in this process",
        if unusable {
            Ok("ring and aws-lc-rs are both compiled in and none is installed".to_string())
        } else {
            Err(
                "rustls picked a provider by itself, so the next two checks prove nothing"
                    .to_string(),
            )
        },
    );

    // A real `wss` handshake with the participant's JSON API, with no TLS
    // settings given: the path that used to build tungstenite's own connector.
    let outcome = async {
        let client = JsonClient::new(env("CANTON_TEST_JSON_ENDPOINT")?)
            .with_oidc(oidc()?)
            .with_timeout(Duration::from_secs(20));
        let party = env("CANTON_TOKEN_SENDER").unwrap_or_else(|_| "nobody::1220".to_string());
        match client.ws_completions(vec![party], 0).await {
            Ok(_stream) => Ok("the wss handshake with the DevNet JSON API completed".to_string()),
            // Reaching the participant and being told no is still a TLS
            // handshake that worked.
            Err(error @ (Error::Http { .. } | Error::Auth(_))) => Ok(format!(
                "TLS worked; the participant answered {}",
                class(&error)
            )),
            Err(error) => Err(format!("{}: {error}", class(&error))),
        }
    }
    .await;
    report.record("tls", "#20 a wss stream with default TLS settings", outcome);

    // PQS over TLS, against a local socket that hangs up: far enough to build
    // the TLS configuration, which is where the panic was.
    let outcome = async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| e.to_string())?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();
        tokio::spawn(async move {
            while let Ok((socket, _)) = listener.accept().await {
                drop(socket);
            }
        });
        let config = format!("host=127.0.0.1 port={port} user=pqs dbname=pqs");
        match canton::pqs::PqsClient::connect_tls_with_timeout(&config, Duration::from_secs(10))
            .await
        {
            Ok(_) => Err("a socket that hangs up answered as a store".to_string()),
            Err(error) if error.to_string().contains("no native root") => Err(format!(
                "no root certificates here, so the TLS setup was not reached: {error}"
            )),
            Err(error) => Ok(format!(
                "no panic; the connection failed as {}",
                class(&error)
            )),
        }
    }
    .await;
    report.record("tls", "#20 a PQS TLS connection", outcome);
    report.record(
        "tls",
        "#20 the SDK installed no process-wide provider",
        if rustls::crypto::CryptoProvider::get_default().is_none() {
            Ok("the default is still the application's to choose".to_string())
        } else {
            Err("a default provider is installed now".to_string())
        },
    );
}

fn class(error: &Error) -> String {
    let text = format!("{error:?}");
    let name = text.split(['(', '{', ' ']).next().unwrap_or_default();
    format!(
        "Error::{name}{}",
        if error.is_retriable() {
            " (retriable)"
        } else {
            ""
        }
    )
}

/// #25: what a failed token fetch is reported as.
async fn auth(report: &mut Report) {
    let outcome = async {
        let unreachable = TokenProvider::new(OidcConfig::new("http://127.0.0.1:1/token", "client", "secret"));
        match unreachable.token().await {
            Err(error @ Error::Connection(_)) if error.is_retriable() => {}
            other => return Err(format!("an unreachable endpoint: {:?}", other.map(|_| "a token"))),
        }
        // A real issuer rejecting a credential stays an authentication error.
        let rejected = TokenProvider::new(OidcConfig::new(
            env("CANTON_TEST_TOKEN_URL")?,
            env("CANTON_TEST_ADMIN_CLIENT_ID")?,
            "not-the-secret",
        ));
        match rejected.token().await {
            Err(Error::Auth(_)) => {}
            Err(other) => return Err(format!("a rejected credential was reported as {}", class(&other))),
            Ok(_) => return Err("the issuer accepted a wrong secret".to_string()),
        }
        let token = oidc()?.token().await.map_err(|e| format!("the real credentials: {}", class(&e)))?;
        if token.len() < 100 {
            return Err("the token is implausibly short".to_string());
        }
        Ok("unreachable is a retriable Connection, a wrong secret is Auth, the right one gets a token".to_string())
    }
    .await;
    report.record("auth", "#25 token fetch failures keep their class", outcome);
}

/// The request with the field Canton 3.7 added: ask for the transaction in
/// the format of an older protocol version.
#[derive(Clone, PartialEq, prost::Message)]
struct GenerateInFormat {
    #[prost(string, tag = "1")]
    synchronizer: String,
    #[prost(string, tag = "2")]
    party_hint: String,
    #[prost(message, optional, tag = "3")]
    public_key: Option<canton::ledger::proto::SigningPublicKey>,
    #[prost(uint32, optional, tag = "8")]
    protocol_version: Option<u32>,
}

/// The envelope a topology transaction travels in: its bytes and its format.
#[derive(Clone, PartialEq, prost::Message)]
struct Envelope {
    #[prost(bytes = "vec", tag = "1")]
    data: Vec<u8>,
    #[prost(int32, tag = "2")]
    version: i32,
}

fn der(raw: &PublicKey) -> PublicKey {
    let mut bytes = vec![
        0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
    ];
    bytes.extend_from_slice(raw.data());
    PublicKey::new(
        KeyFormat::DerX509SubjectPublicKeyInfo,
        bytes,
        KeySpec::EcCurve25519,
    )
}

/// #22, the topology formats, and onboarding in two sittings, on DevNet.
/// Returns a party that can sign, for the submission checks.
async fn onboarding(
    report: &mut Report,
) -> Option<(CantonClient, String, canton::signer::Ed25519Signer)> {
    let setup = async {
        let endpoint = env("CANTON_TEST_ENDPOINT")?;
        let admin = AdminClient::connect_lazy(
            canton::admin::Config::new(endpoint.clone()).with_oidc(oidc()?),
        )
        .map_err(|e| e.to_string())?;
        let ledger =
            CantonClient::connect_lazy(canton::ledger::Config::new(endpoint).with_oidc(oidc()?))
                .map_err(|e| e.to_string())?;
        let version = ledger.version().await.map_err(|e| e.to_string())?;
        let synchronizer = ledger
            .connected_synchronizers("")
            .await
            .map_err(|e| e.to_string())?
            .first()
            .ok_or("no synchronizer")?
            .synchronizer_id
            .clone();
        let participant = admin.participant_id().await.map_err(|e| e.to_string())?;
        let user = admin.current_user().await.map_err(|e| e.to_string())?.id;
        Ok::<_, String>((admin, ledger, version, synchronizer, participant, user))
    }
    .await;
    let (admin, ledger, version, synchronizer, participant, user) = match setup {
        Ok(setup) => setup,
        Err(error) => {
            report.record("onboarding", "reach the participant", Err(error));
            return None;
        }
    };
    report.record(
        "onboarding",
        "reach the participant",
        Ok(format!("Canton {version}")),
    );
    let expected = ExpectedHosting::participant(participant.clone());
    let key = Ed25519Key::generate().expect("a random source");
    let raw = key.public_key();
    let fingerprint = raw.fingerprint().expect("an Ed25519 key");

    // The reply a participant really sends, held to the strict check, for a
    // raw key and for the same key as DER.
    let generated = async {
        let hint = format!("verify-{}", &uuid::Uuid::new_v4().simple().to_string()[..8]);
        let from_raw = admin
            .generate_external_party_topology_with(&synchronizer, &hint, &raw, &expected)
            .await
            .map_err(|e| format!("a raw key: {e}"))?;
        let from_der = admin
            .generate_external_party_topology_with(&synchronizer, &hint, &der(&raw), &expected)
            .await
            .map_err(|e| format!("a DER key: {e}"))?;
        if from_raw.public_key_fingerprint() != fingerprint
            || from_der.public_key_fingerprint() != fingerprint
        {
            return Err("the fingerprint is not the one computed from the key".to_string());
        }
        if from_raw.party_id() != format!("{hint}::{fingerprint}")
            || from_raw.party_id() != from_der.party_id()
        {
            return Err(format!("unexpected party {}", from_raw.party_id()));
        }
        if !participant.ends_with(from_raw.hosting_participant()) {
            return Err(format!("hosted by {}", from_raw.hosting_participant()));
        }
        let format = Envelope::decode(from_raw.transactions()[0].as_slice())
            .map_err(|e| e.to_string())?
            .version;
        Ok((from_raw, hint, format))
    }
    .await;
    let (generated, hint, format) = match generated {
        Ok(ok) => {
            report.record(
                "onboarding",
                "#22 a real reply passes the strict check, raw and DER key",
                Ok(format!(
                    "topology format {}, fingerprint computed locally, host is this participant",
                    ok.2
                )),
            );
            ok
        }
        Err(error) => {
            report.record(
                "onboarding",
                "#22 a real reply passes the strict check, raw and DER key",
                Err(error),
            );
            return None;
        }
    };

    // Real bytes, tampered with: none of it may pass.
    let party = generated.party_id().to_string();
    let honest = generated.transactions()[0].clone();
    let outcome = (|| {
        let refuse = |what: &str,
                      bytes: Vec<u8>,
                      key: &PublicKey,
                      expected: &ExpectedHosting,
                      party: &str| {
            match ExternalPartyTopology::from_parts(party, vec![bytes], key, expected) {
                Err(Error::UnexpectedResponse(_)) => Ok(()),
                Err(other) => Err(format!("{what}: refused as {}", class(&other))),
                Ok(_) => Err(format!("{what}: ACCEPTED")),
            }
        };
        let mut flipped = 0;
        for at in 0..honest.len() {
            let mut bytes = honest.clone();
            bytes[at] ^= 0x01;
            refuse(
                &format!("byte {at} changed"),
                bytes,
                &raw,
                &expected,
                &party,
            )?;
            flipped += 1;
        }
        let mut extended = honest.clone();
        extended.extend_from_slice(&[0x78, 0x01]);
        refuse(
            "an unknown field appended",
            extended,
            &raw,
            &expected,
            &party,
        )?;
        let mut envelope = Envelope::decode(honest.as_slice()).map_err(|e| e.to_string())?;
        envelope.data.extend_from_slice(&[0x78, 0x01]);
        refuse(
            "an unknown field inside",
            envelope.encode_to_vec(),
            &raw,
            &expected,
            &party,
        )?;
        let other = Ed25519Key::generate()
            .expect("a random source")
            .public_key();
        refuse("another key", honest.clone(), &other, &expected, &party)?;
        refuse(
            "another host",
            honest.clone(),
            &raw,
            &ExpectedHosting::participant("someone-else::1220ffff"),
            &party,
        )?;
        refuse(
            "another party",
            honest.clone(),
            &raw,
            &expected,
            &format!("mallory::{fingerprint}"),
        )?;
        match ExternalPartyTopology::from_parts(
            &party,
            vec![honest.clone(), honest.clone()],
            &raw,
            &expected,
        ) {
            Err(Error::UnexpectedResponse(_)) => {}
            _ => return Err("two transactions were not refused".to_string()),
        }
        Ok(format!(
            "each of {flipped} single-byte changes refused, and 6 other tamperings"
        ))
    })();
    report.record(
        "onboarding",
        "#22 a tampered real transaction never passes",
        outcome,
    );

    // The older format, produced by the same participant on request.
    let outcome = async {
        let endpoint = env("CANTON_TEST_ENDPOINT")?;
        let token = oidc()?.token().await.map_err(|e| e.to_string())?;
        let channel = tonic::transport::Channel::from_shared(endpoint)
            .map_err(|e| e.to_string())?
            .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
            .map_err(|e| e.to_string())?
            .connect()
            .await
            .map_err(|e| e.to_string())?;
        let mut grpc = tonic::client::Grpc::new(channel);
        grpc.ready().await.map_err(|e| e.to_string())?;
        let mut request = tonic::Request::new(GenerateInFormat {
            synchronizer: synchronizer.clone(),
            party_hint: hint.clone(),
            public_key: Some((&raw).into()),
            protocol_version: Some(36),
        });
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {token}")
                .parse()
                .map_err(|_| "a token that is not a header value")?,
        );
        let path =
            "/com.daml.ledger.api.v2.admin.PartyManagementService/GenerateExternalPartyTopology"
                .parse()
                .map_err(|_| "a path")?;
        let reply: canton::ledger::proto::admin::GenerateExternalPartyTopologyResponse = grpc
            .unary(request, path, tonic_prost::ProstCodec::default())
            .await
            .map_err(|status| {
                format!(
                    "the participant refused protocol version 36: {}",
                    status.message()
                )
            })?
            .into_inner();
        let format = Envelope::decode(
            reply
                .topology_transactions
                .first()
                .ok_or("no transaction")?
                .as_slice(),
        )
        .map_err(|e| e.to_string())?
        .version;
        if format != 30 {
            return Err(format!(
                "asked for protocol version 36 and got topology format {format}"
            ));
        }
        let rebuilt = ExternalPartyTopology::from_parts(
            &reply.party_id,
            reply.topology_transactions.clone(),
            &raw,
            &expected,
        )
        .map_err(|e| format!("Canton's own format-30 transaction was refused: {e}"))?;
        if rebuilt.multi_hash() != reply.multi_hash.as_slice() {
            return Err("the multi-hash differs from the participant's".to_string());
        }
        Ok(
            "a format-30 transaction from the same participant passes, and the multi-hash matches"
                .to_string(),
        )
    }
    .await;
    report.record(
        "onboarding",
        "topology format 30, as Canton writes it",
        outcome,
    );

    // Two sittings: store, sign elsewhere, rebuild, allocate.
    let stored_party = generated.party_id().to_string();
    let stored_transactions = generated.transactions().to_vec();
    let hash = generated.multi_hash().to_vec();
    drop(generated);
    let signer = key.into_signer(fingerprint.clone());
    let allocated = async {
        let signature = signer.sign(&hash).await.map_err(|e| e.to_string())?;
        let rebuilt =
            ExternalPartyTopology::from_parts(&stored_party, stored_transactions, &raw, &expected)
                .map_err(|e| format!("rebuilding what was stored: {e}"))?;
        // A signature that names another key never leaves this process.
        let stranger = Ed25519Key::generate().expect("a random source");
        let stranger = stranger.into_signer("1220deadbeef");
        let wrong = stranger.sign(&hash).await.map_err(|e| e.to_string())?;
        match admin
            .allocate_external_party_with_signature(&synchronizer, &rebuilt, &wrong, &user)
            .await
        {
            Err(Error::InvalidRequest(_)) => {}
            other => {
                return Err(format!(
                    "a signature by another key: {:?}",
                    other.map(|_| "allocated")
                ));
            }
        }
        let party = admin
            .allocate_external_party_with_signature(&synchronizer, &rebuilt, &signature, &user)
            .await
            .map_err(|e| format!("allocating: {e}"))?;
        if party != stored_party {
            return Err(format!("allocated {party}"));
        }
        Ok(party)
    }
    .await;
    match allocated {
        Ok(party) => {
            report.record(
                "onboarding",
                "a stored topology is rebuilt and allocated with a signature made later",
                Ok(format!(
                    "allocated {}…, topology format {format}",
                    &party[..party.len().min(28)]
                )),
            );
            Some((ledger, party, signer))
        }
        Err(error) => {
            report.record(
                "onboarding",
                "a stored topology is rebuilt and allocated with a signature made later",
                Err(error),
            );
            None
        }
    }
}

fn ping(party: &str) -> canton::ledger::proto::Command {
    canton::ledger::create(
        identifier(
            "#canton-builtin-admin-workflow-ping",
            "Canton.Internal.Ping",
            "Ping",
        ),
        record(vec![
            (
                "id",
                value::text(format!("verify-{}", uuid::Uuid::new_v4())),
            ),
            ("initiator", value::party(party)),
            ("responder", value::party(party)),
        ]),
    )
}

/// #24 and #26 through a real prepare, sign and execute.
async fn submission(
    report: &mut Report,
    ledger: &CantonClient,
    party: &str,
    signer: &canton::signer::Ed25519Signer,
) {
    let outcome = async {
        let prepared = ledger
            .prepare_submission(
                Prepare::new(party)
                    .add_command(ping(party))
                    .with_command_id("verify-command-id"),
            )
            .await
            .map_err(|e| format!("prepare: {e}"))?;
        if prepared.hashing_scheme_version() != V2 {
            return Err(format!(
                "the default scheme is {:?}",
                prepared.hashing_scheme_version()
            ));
        }
        let executable = prepared
            .sign_with(signer)
            .await
            .map_err(|e| e.to_string())?;
        let transaction = Box::pin(ledger.execute_submission_and_wait_for_transaction(executable))
            .await
            .map_err(|e| format!("execute: {e}"))?;
        Ok(format!("V2, committed at offset {}", transaction.offset))
    }
    .await;
    report.record(
        "submission",
        "#24 a real prepare names the command and parties that were asked for",
        outcome,
    );

    let outcome = async {
        let in_ten_minutes = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs()
            + 600;
        let deadline = prost_timestamp(in_ten_minutes);
        match ledger
            .prepare_submission(
                Prepare::new(party)
                    .add_command(ping(party))
                    .with_max_record_time(deadline)
                    .with_hashing_scheme_version(V2),
            )
            .await
        {
            Err(Error::InvalidRequest(_)) => {}
            other => return Err(format!("a deadline under a chosen V2: {:?}", other.map(|_| "prepared"))),
        }
        let prepared = ledger
            .prepare_submission(Prepare::new(party).add_command(ping(party)).with_max_record_time(deadline))
            .await
            .map_err(|e| format!("prepare with a deadline: {e}"))?;
        if prepared.hashing_scheme_version() != V3 {
            return Err(format!("a deadline was prepared under {:?}", prepared.hashing_scheme_version()));
        }
        let executable = prepared.sign_with(signer).await.map_err(|e| e.to_string())?;
        let transaction = Box::pin(ledger.execute_submission_and_wait_for_transaction(executable))
            .await
            .map_err(|e| format!("execute: {e}"))?;
        Ok(format!("V3 without being asked, hash matched, committed at offset {}; V2 with a deadline refused before sending", transaction.offset))
    }
    .await;
    report.record(
        "submission",
        "#26 a max record time is prepared and committed under V3",
        outcome,
    );
}

fn prost_timestamp(seconds: u64) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: i64::try_from(seconds).unwrap_or(i64::MAX),
        nanos: 0,
    }
}

/// #21: the registry client after its transport was shared, and the Scan
/// client against whatever answers at the configured Scan.
async fn scan(report: &mut Report) {
    let outcome = async {
        let registry = canton::token::RegistryClient::new(&env("CANTON_TOKEN_REGISTRY_URL")?)
            .map_err(|e| e.to_string())?;
        let info = registry
            .info()
            .await
            .map_err(|e| format!("{}: {e}", class(&e)))?;
        if !info.admin_id.contains("::1220") {
            return Err(format!(
                "an admin id that is not a party: {}",
                info.admin_id
            ));
        }
        let missing = registry
            .instrument("no-such-instrument")
            .await
            .map_err(|e| e.to_string())?;
        if missing.is_some() {
            return Err("an instrument nobody issues".to_string());
        }
        Ok(
            "the live registry names its admin party, and an unknown instrument is None"
                .to_string(),
        )
    }
    .await;
    report.record(
        "scan",
        "#21 the registry client still reads a live registry",
        outcome,
    );

    let outcome = async {
        let base = format!("{}/api/scan", env("CANTON_TOKEN_REGISTRY_URL")?.trim_end_matches('/'));
        let client = canton::token::scan::ScanClient::new(&base).map_err(|e| e.to_string())?;
        // Refused before anything is sent, whatever the Scan would say.
        match client.featured_app_right("nobody::1220").await {
            Err(Error::InvalidRequest(_)) => {}
            other => return Err(format!("an internal endpoint without the opt-in: {:?}", other.map(|_| "answered"))),
        }
        match client.dso_party_id().await {
            Ok(dso) if dso.contains("::1220") => {
                let rounds = client.open_and_issuing_mining_rounds().await.map_err(|e| e.to_string())?;
                let round = client.latest_open_mining_round().await.map_err(|e| e.to_string())?;
                round.disclosed().map_err(|e| e.to_string())?;
                let rules = client.amulet_rules().await.map_err(|e| e.to_string())?;
                rules.disclosed().map_err(|e| e.to_string())?;
                Ok(format!("a live Scan: DSO party, {} open rounds, the rules and the round both disclose", rounds.open.len()))
            }
            Ok(other) => Err(format!("a DSO party that is not one: {other}")),
            // This Scan answers only to allow-listed addresses, and this
            // machine is not one. The client must say so plainly.
            Err(Error::Http { status: 403, .. }) => Ok(
                "NOT VERIFIED LIVE: this Scan answers 403 to this address (allow-list); the refusal is reported as Error::Http 403, not retried"
                    .to_string(),
            ),
            Err(error) => Err(format!("{}: {error}", class(&error))),
        }
    }
    .await;
    report.record(
        "scan",
        "#21 the Scan client against the configured Scan",
        outcome,
    );
}

#[tokio::main]
async fn main() {
    // The checks catch panics to report them; the default hook would print
    // each one's backtrace as well.
    std::panic::set_hook(Box::new(|_| {}));
    let mut report = Report {
        failed: 0,
        passed: 0,
    };

    hashing(&mut report);
    tls_providers(&mut report).await;
    auth(&mut report).await;
    if let Some((ledger, party, signer)) = onboarding(&mut report).await {
        submission(&mut report, &ledger, &party, &signer).await;
    } else {
        report.record(
            "submission",
            "prepare, sign and execute",
            Err("no onboarded party to submit as".to_string()),
        );
    }
    scan(&mut report).await;

    println!("\n{} passed, {} failed", report.passed, report.failed);
    if report.failed > 0 {
        std::process::exit(1);
    }
}
