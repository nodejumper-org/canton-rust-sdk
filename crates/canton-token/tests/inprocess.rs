//! The whole transfer path, against a registry standing in this process.
//!
//! A real registry needs a Splice scan, and a LocalNet running only a validator
//! has none — so without this the HTTP layer would be exercised nowhere. The
//! stub answers exactly what the token standard's OpenAPI says a registry
//! answers, and *records what it was asked*, which is the half that matters:
//! the specification is precise about what a client must send, and a client
//! that sends something else gets a context for a question it did not ask.
//!
//! Hand-rolled HTTP rather than a server framework: one request, one response,
//! and no dependency the published crate does not already carry.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};

use canton_daml as rt;
use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1 as h;
use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;
use canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1 as ti;
use canton_token::RegistryClient;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

/// What the stub was asked, so a test can assert on the request rather than
/// only on the reply.
#[derive(Clone, Debug, Default)]
struct Recorded {
    /// The HTTP verb. Recorded because the standard specifies one per endpoint
    /// — `POST` for every factory and choice context, `GET` for metadata — and
    /// without it a factory issued as a `GET` passed every assertion here:
    /// `reqwest` sends the body either way, so the path and body both matched.
    method: String,
    path: String,
    body: serde_json::Value,
}

/// Serve `response` once, recording the request. Returns the base URL.
async fn registry(response: serde_json::Value) -> (String, Arc<Mutex<Recorded>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let recorded = Arc::new(Mutex::new(Recorded::default()));
    let seen = Arc::clone(&recorded);

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let mut buffer = Vec::new();
        let mut chunk = [0u8; 4096];

        // Read until the headers are complete, then until the body is.
        loop {
            let read = socket.read(&mut chunk).await.expect("read");
            if read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..read]);
            let text = String::from_utf8_lossy(&buffer).to_string();
            let Some(head_end) = text.find("\r\n\r\n") else {
                continue;
            };
            let head = &text[..head_end];
            let body = &text[head_end + 4..];
            let content_length: usize = head
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse().ok())?
                })
                .unwrap_or(0);
            if body.len() < content_length {
                continue;
            }

            let request_line = head.lines().next().unwrap_or_default();
            let method = request_line
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string();
            let path = request_line
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_string();
            *seen.lock().expect("lock") = Recorded {
                method,
                path,
                body: serde_json::from_str(body).unwrap_or(serde_json::Value::Null),
            };
            break;
        }

        let payload = response.to_string();
        let http = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{payload}",
            payload.len()
        );
        socket.write_all(http.as_bytes()).await.expect("write");
        socket.flush().await.expect("flush");
    });

    (format!("http://127.0.0.1:{port}"), recorded)
}

fn party(s: &str) -> rt::Party {
    rt::Party::parse(s).expect("a party")
}

fn a_transfer() -> ti::Transfer {
    ti::Transfer {
        sender: party("alice::1220ab"),
        receiver: party("bob::1220cd"),
        amount: "10.5".parse().expect("a numeric"),
        instrument_id: h::InstrumentId {
            admin: party("dso::1220ef"),
            id: "Amulet".to_string(),
        },
        requested_at: rt::Timestamp(1_700_000_000_000_000),
        execute_before: rt::Timestamp(1_700_000_600_000_000),
        input_holding_cids: vec![rt::ContractId::new("00holding")],
        meta: md::Metadata {
            values: rt::TextMap::new(),
        },
    }
}

fn a_factory_response() -> serde_json::Value {
    serde_json::json!({
        "factoryId": "00factory",
        "transferKind": "direct",
        "choiceContext": {
            "choiceContextData": {
                "values": {
                    "amulet-rules": { "tag": "AV_ContractId", "value": "00rules" },
                    "open-round":   { "tag": "AV_ContractId", "value": "00round" }
                }
            },
            "disclosedContracts": [
                {
                    "templateId": "pkgA:Splice.AmuletRules:AmuletRules",
                    "contractId": "00rules",
                    "createdEventBlob": "AQID",
                    "synchronizerId": "sync::1220ab"
                },
                {
                    "templateId": "pkgA:Splice.Round:OpenMiningRound",
                    "contractId": "00round",
                    "createdEventBlob": "BAUG",
                    "synchronizerId": "sync::1220ab"
                }
            ]
        }
    })
}

/// The path is the standard's, and the body is the choice as the Daml JSON API
/// encodes it with empty `extraArgs` — which the specification states outright.
/// A registry picks the context from these arguments, so sending the wrong
/// shape yields a context for a different question.
#[tokio::test]
async fn the_factory_is_asked_the_way_the_standard_says_to_ask() {
    let (base, recorded) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");

    canton_token::transfer(&client, &party("dso::1220ef"), a_transfer())
        .await
        .expect("the transfer resolves");

    let seen = recorded.lock().expect("lock").clone();
    assert_eq!(seen.method, "POST", "the standard specifies POST here");
    assert_eq!(
        seen.path,
        "/registry/transfer-instruction/v1/transfer-factory"
    );

    let arguments = &seen.body["choiceArguments"];
    assert_eq!(arguments["expectedAdmin"], "dso::1220ef");
    assert_eq!(arguments["transfer"]["sender"], "alice::1220ab");
    assert_eq!(arguments["transfer"]["receiver"], "bob::1220cd");
    // A Numeric is a string in LF-JSON, not a number.
    assert_eq!(arguments["transfer"]["amount"], "10.5");
    assert_eq!(
        arguments["transfer"]["inputHoldingCids"],
        serde_json::json!(["00holding"])
    );
    assert_eq!(
        arguments["extraArgs"],
        serde_json::json!({ "context": { "values": {} }, "meta": { "values": {} } }),
        "the standard says the probe carries empty extraArgs"
    );
}

/// Everything the registry returned reaches the command: the context into the
/// choice, the contracts onto the submission. Dropping either produces a
/// submission that fails at interpretation for a reason that names neither.
#[tokio::test]
async fn what_the_registry_returns_reaches_the_ledger() {
    let (base, _) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");

    let command = canton_token::transfer(&client, &party("dso::1220ef"), a_transfer())
        .await
        .expect("the transfer resolves");

    assert_eq!(
        command.transfer_kind(),
        Some(canton_token::TransferKind::Direct)
    );

    // Both disclosures, with their blobs decoded from base64 for gRPC.
    let disclosed = command.disclosed_contracts();
    assert_eq!(disclosed.len(), 2);
    assert_eq!(disclosed[0].contract_id, "00rules");
    assert_eq!(disclosed[0].created_event_blob, [1, 2, 3]);
    assert_eq!(disclosed[1].created_event_blob, [4, 5, 6]);
    let id = disclosed[0].template_id.as_ref().expect("a template id");
    assert_eq!(id.package_id, "pkgA");
    assert_eq!(id.module_name, "Splice.AmuletRules");
    assert_eq!(id.entity_name, "AmuletRules");

    // The exercise names the factory the registry returned, and the choice the
    // interface declares.
    let exercised = match command.command().command.as_ref().expect("a command") {
        canton_proto::com::daml::ledger::api::v2::command::Command::Exercise(e) => e,
        other => panic!("expected an exercise, got {other:?}"),
    };
    assert_eq!(exercised.contract_id, "00factory");
    assert_eq!(exercised.choice, "TransferFactory_Transfer");

    // And the context is inside the choice argument, not merely alongside it.
    let argument = format!("{:?}", exercised.choice_argument);
    assert!(
        argument.contains("amulet-rules") && argument.contains("open-round"),
        "the registry's context must be in the choice argument: {argument}"
    );
}

/// The disclosures travel with the command into a submission, on both paths.
#[tokio::test]
async fn the_disclosures_are_attached_to_whichever_submission_is_used() {
    let (base, _) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");
    let command = canton_token::transfer(&client, &party("dso::1220ef"), a_transfer())
        .await
        .expect("the transfer resolves");

    let prepare = command.into_prepare("alice::1220ab");
    assert_eq!(
        prepare.disclosed_contracts().len(),
        2,
        "an interactively signed transfer needs them just as much"
    );
}

/// A registry that fails says why, and the error carries exactly that: the
/// body is the registry's message, so the shared error model can read it.
#[tokio::test]
async fn a_failing_registry_reports_its_own_message() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let mut chunk = [0u8; 4096];
        let _ = socket.read(&mut chunk).await;
        let body = r#"{"error":"no factory for instrument Amulet"}"#;
        let http = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );
        socket.write_all(http.as_bytes()).await.expect("write");
    });

    let client = RegistryClient::new(&format!("http://127.0.0.1:{port}")).expect("client");
    let err = canton_token::transfer(&client, &party("dso::1220ef"), a_transfer())
        .await
        .expect_err("the registry refused");

    let message = err.to_string();
    assert!(
        message.contains("no factory for instrument Amulet"),
        "{message}"
    );
    assert!(message.contains("transfer-factory"), "{message}");
    assert!(
        matches!(&err, canton_core::Error::Http { status: 404, body, ..  } if body == "no factory for instrument Amulet"),
        "{err:?}"
    );
}

/// The metadata API, read the way the standard publishes it.
#[tokio::test]
async fn the_registry_describes_itself_and_its_instruments() {
    let (base, _) = registry(serde_json::json!({
        "adminId": "dso::1220ef",
        "supportedApis": { "splice-api-token-metadata-v1": 1 }
    }))
    .await;
    let info = RegistryClient::new(&base)
        .expect("client")
        .info()
        .await
        .expect("info");
    assert_eq!(info.admin_id, "dso::1220ef");
    assert_eq!(info.supported_apis["splice-api-token-metadata-v1"], 1);

    let (base, recorded) = registry(serde_json::json!({
        "instruments": [{ "id": "Amulet", "name": "Canton Coin", "symbol": "CC", "decimals": 10 }],
        "nextPageToken": "page-2"
    }))
    .await;
    let (instruments, next) = RegistryClient::new(&base)
        .expect("client")
        .list_instruments(Some(50), None)
        .await
        .expect("instruments");
    assert_eq!(instruments.len(), 1);
    assert_eq!(instruments[0].symbol, "CC");
    assert_eq!(instruments[0].decimals, 10);
    assert_eq!(next.as_deref(), Some("page-2"));
    assert!(
        recorded.lock().expect("lock").path.contains("pageSize=50"),
        "the page size must reach the query string"
    );
}

/// Every registry path, observed. `/registry/allocations/v1/` is plural and
/// `/registry/allocation-instruction/v1/` is singular — the two easiest things
/// in this crate to get wrong, and only the transfer factory was checked.
#[tokio::test]
async fn every_registry_path_is_the_one_the_standard_publishes() {
    let empty = serde_json::json!({
        "factoryId": "00f",
        "choiceContext": { "choiceContextData": {}, "disclosedContracts": [] }
    });
    let (base, recorded) = registry(empty).await;
    canton_token::RegistryClient::new(&base)
        .expect("client")
        .allocation_factory(&serde_json::json!({}))
        .await
        .expect("resolves");
    assert_eq!(
        recorded.lock().expect("lock").path,
        "/registry/allocation-instruction/v1/allocation-factory"
    );

    let context = serde_json::json!({ "choiceContextData": {}, "disclosedContracts": [] });
    for (choice, expected) in [
        (
            canton_token::AllocationChoice::ExecuteTransfer,
            "/registry/allocations/v1/00alloc/choice-contexts/execute-transfer",
        ),
        (
            canton_token::AllocationChoice::Withdraw,
            "/registry/allocations/v1/00alloc/choice-contexts/withdraw",
        ),
        (
            canton_token::AllocationChoice::Cancel,
            "/registry/allocations/v1/00alloc/choice-contexts/cancel",
        ),
    ] {
        let (base, recorded) = registry(context.clone()).await;
        canton_token::RegistryClient::new(&base)
            .expect("client")
            .allocation_context(
                "00alloc",
                choice,
                &canton_token::ChoiceContextRequest::default(),
            )
            .await
            .expect("resolves");
        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{expected} is a POST endpoint");
        assert_eq!(seen.path, expected);
    }

    for (choice, expected) in [
        (
            canton_token::TransferInstructionChoice::Accept,
            "/registry/transfer-instruction/v1/00inst/choice-contexts/accept",
        ),
        (
            canton_token::TransferInstructionChoice::Reject,
            "/registry/transfer-instruction/v1/00inst/choice-contexts/reject",
        ),
        (
            canton_token::TransferInstructionChoice::Withdraw,
            "/registry/transfer-instruction/v1/00inst/choice-contexts/withdraw",
        ),
    ] {
        let (base, recorded) = registry(context.clone()).await;
        canton_token::RegistryClient::new(&base)
            .expect("client")
            .transfer_instruction_context(
                "00inst",
                choice,
                &canton_token::ChoiceContextRequest::default(),
            )
            .await
            .expect("resolves");
        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{expected} is a POST endpoint");
        assert_eq!(seen.path, expected);
    }
}

/// The V2 paths are not V1's with a digit changed. Every collection here is
/// spelled in the **singular** — `transfer-instruction`, `allocation-instruction`,
/// `allocation` — except the choice contexts on an allocation, which are under
/// the **plural** `allocations`. A registry that is working correctly answers
/// 404 to either spelling in the wrong place, so this pins each one against the
/// OpenAPI documents rather than against a rule about how they read.
#[tokio::test]
async fn the_v2_paths_are_the_ones_the_standard_publishes_odd_pluralisation_included() {
    let factory = serde_json::json!({
        "factoryId": "00f",
        "transferKind": "offer",
        "choiceContext": { "choiceContextData": {}, "disclosedContracts": [] }
    });
    for (expected, call) in [
        (
            "/registry/transfer-instruction/v2/transfer-factory",
            0_u8, // transfer factory
        ),
        ("/registry/allocation-instruction/v2/allocation-factory", 1),
        ("/registry/allocation/v2/settlement-factory", 2),
    ] {
        let (base, recorded) = registry(factory.clone()).await;
        let client = RegistryClient::new(&base).expect("client");
        let args = serde_json::json!({});
        match call {
            0 => client.transfer_factory_v2(&args).await.map(|_| ()),
            1 => client.allocation_factory_v2(&args).await.map(|_| ()),
            _ => client.settlement_factory_v2(&args).await.map(|_| ()),
        }
        .expect("resolves");
        assert_eq!(recorded.lock().expect("lock").path, expected);
    }

    let context = serde_json::json!({ "choiceContextData": {}, "disclosedContracts": [] });
    for (choice, expected) in [
        (
            canton_token::AllocationChoice::Withdraw,
            "/registry/allocations/v2/00alloc/choice-contexts/withdraw",
        ),
        (
            canton_token::AllocationChoice::Cancel,
            "/registry/allocations/v2/00alloc/choice-contexts/cancel",
        ),
    ] {
        let (base, recorded) = registry(context.clone()).await;
        RegistryClient::new(&base)
            .expect("client")
            .allocation_context_v2(
                "00alloc",
                choice,
                &canton_token::ChoiceContextRequest::default(),
            )
            .await
            .expect("resolves");
        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{expected} is a POST endpoint");
        assert_eq!(seen.path, expected);
    }

    // Contexts V1 has no counterpart for.
    for (choice, expected) in [
        (
            canton_token::AllocationInstructionChoice::Accept,
            "/registry/allocation-instruction/v2/00ai/choice-contexts/accept",
        ),
        (
            canton_token::AllocationInstructionChoice::Withdraw,
            "/registry/allocation-instruction/v2/00ai/choice-contexts/withdraw",
        ),
    ] {
        let (base, recorded) = registry(context.clone()).await;
        RegistryClient::new(&base)
            .expect("client")
            .allocation_instruction_context_v2(
                "00ai",
                choice,
                &canton_token::ChoiceContextRequest::default(),
            )
            .await
            .expect("resolves");
        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{expected} is a POST endpoint");
        assert_eq!(seen.path, expected);
    }
}

/// V2 settles a batch through the settlement factory. Asking for an
/// execute-transfer context is a V1 habit, and it has to fail here rather than
/// reach the network — a registry would answer 404 with nothing that says why.
#[tokio::test]
async fn a_v1_execute_transfer_context_is_refused_before_it_is_sent() {
    let (base, recorded) = registry(serde_json::json!({})).await;
    let error = RegistryClient::new(&base)
        .expect("client")
        .allocation_context_v2(
            "00alloc",
            canton_token::AllocationChoice::ExecuteTransfer,
            &canton_token::ChoiceContextRequest::default(),
        )
        .await
        .expect_err("V2 has no execute-transfer context");

    assert!(
        matches!(error, canton_core::Error::InvalidRequest(_)),
        "expected the request to be refused, got {error:?}"
    );
    assert!(
        error.to_string().contains("settlement factory"),
        "the message must name the replacement: {error}"
    );
    assert_eq!(
        recorded.lock().expect("lock").path,
        "",
        "nothing may reach the network"
    );
}

/// A page token is an instrument id by the standard's own definition, so it can
/// contain anything an id can. Formatted into the query string, one containing
/// `&` split into extra parameters; this is the branch that was never sent.
#[tokio::test]
async fn a_page_token_is_encoded_rather_than_formatted() {
    let (base, recorded) = registry(serde_json::json!({ "instruments": [] })).await;
    canton_token::RegistryClient::new(&base)
        .expect("client")
        .list_instruments(Some(10), Some("a&pageSize=1&b"))
        .await
        .expect("the query runs");

    let path = recorded.lock().expect("lock").path.clone();
    assert!(
        path.contains("pageToken=a%26pageSize%3D1%26b"),
        "the token must be one encoded value: {path}"
    );
    assert_eq!(
        path.matches("pageSize=").count(),
        1,
        "a token cannot introduce a second pageSize: {path}"
    );
}

/// A token-standard command is meaningless without its disclosures, and both
/// submission paths must carry them. The ordinary one could not be checked at
/// all until `Submit` gained a reader, so a `TokenCommand` that dropped every
/// disclosure on that path would have left the suite green.
#[tokio::test]
async fn the_ordinary_submission_path_carries_the_disclosures_too() {
    let (base, _) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");
    let command = canton_token::transfer(&client, &party("dso::1220ef"), a_transfer())
        .await
        .expect("the transfer resolves");

    let submit = command.into_submit("alice::1220ab");
    assert_eq!(submit.disclosed_contracts().len(), 2);
}

/// Every path this crate spells exists in the token standard's OpenAPI
/// documents, at the revision vendored under `testdata/openapi/` (see its
/// PROVENANCE.md). The shapes below are the ones the stub-based tests above
/// assert the crate sends, with the instance ids replaced by `{}`; a document
/// revision that moves a path fails here rather than against a live registry.
#[test]
fn every_path_the_crate_spells_is_in_the_vendored_specification() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testdata/openapi");
    let mut published = std::collections::BTreeSet::new();
    for name in [
        "token-metadata-v1.yaml",
        "transfer-instruction-v1.yaml",
        "allocation-v1.yaml",
        "allocation-instruction-v1.yaml",
        "transfer-instruction-v2.yaml",
        "allocation-v2.yaml",
        "allocation-instruction-v2.yaml",
    ] {
        let text = std::fs::read_to_string(dir.join(name)).expect(name);
        let mut in_paths = false;
        for line in text.lines() {
            if line.starts_with("paths:") {
                in_paths = true;
                continue;
            }
            if in_paths && !line.is_empty() && !line.starts_with(' ') {
                in_paths = false;
            }
            if in_paths && line.starts_with("  /") && line.ends_with(':') {
                published.insert(normalise(line.trim().trim_end_matches(':')));
            }
        }
    }
    assert!(
        published.len() >= 20,
        "the documents list {} paths",
        published.len()
    );

    let spelled = [
        "/registry/metadata/v1/info",
        "/registry/metadata/v1/instruments",
        "/registry/metadata/v1/instruments/{}",
        "/registry/transfer-instruction/v1/transfer-factory",
        "/registry/transfer-instruction/v1/{}/choice-contexts/accept",
        "/registry/transfer-instruction/v1/{}/choice-contexts/reject",
        "/registry/transfer-instruction/v1/{}/choice-contexts/withdraw",
        "/registry/allocation-instruction/v1/allocation-factory",
        "/registry/allocations/v1/{}/choice-contexts/execute-transfer",
        "/registry/allocations/v1/{}/choice-contexts/withdraw",
        "/registry/allocations/v1/{}/choice-contexts/cancel",
        "/registry/transfer-instruction/v2/transfer-factory",
        "/registry/transfer-instruction/v2/{}/choice-contexts/accept",
        "/registry/transfer-instruction/v2/{}/choice-contexts/reject",
        "/registry/transfer-instruction/v2/{}/choice-contexts/withdraw",
        "/registry/allocation-instruction/v2/allocation-factory",
        "/registry/allocation-instruction/v2/{}/choice-contexts/accept",
        "/registry/allocation-instruction/v2/{}/choice-contexts/withdraw",
        "/registry/allocation/v2/settlement-factory",
        "/registry/allocations/v2/{}/choice-contexts/withdraw",
        "/registry/allocations/v2/{}/choice-contexts/cancel",
    ];
    let missing: Vec<_> = spelled
        .iter()
        .filter(|path| !published.contains(**path))
        .collect();
    assert!(
        missing.is_empty(),
        "paths this crate spells that the vendored documents do not publish: {missing:#?}"
    );
}

/// `{instrumentId}`, `{allocationId}`, … all become `{}`: the name of a path
/// parameter is documentation, its position is the contract.
fn normalise(path: &str) -> String {
    let mut out = String::new();
    let mut in_param = false;
    for c in path.chars() {
        match c {
            '{' => {
                in_param = true;
                out.push_str("{}");
            }
            '}' => in_param = false,
            _ if in_param => {}
            _ => out.push(c),
        }
    }
    out
}

// ---- Every workflow, driven through the stub --------------------------------
//
// The tests above pin the registry client's paths and the V1 transfer. The
// workflow functions built on the client — V1 allocation, the instruction
// choices, and all of V2 — were exercised only by the live examples, which CI
// cannot run. Each one is driven here: what it asks the registry, and whether
// what the registry answered made it into the exercise it built. The second
// half is the one a mutation test found unguarded: a workflow that drops the
// context builds a command that fails at interpretation, with a message that
// names neither the context nor the registry.

use canton_proto::com::daml::ledger::api::v2::command::Command as PbCommand;
use canton_token::types::v1::allocation as al1;
use canton_token::types::v2::{
    allocation as al2, allocation_instruction as ai2, holding as h2, transfer_instruction as ti2,
};

fn no_meta() -> md::Metadata {
    md::Metadata {
        values: rt::TextMap::new(),
    }
}

/// A choice context on its own, as the choice-context endpoints answer.
fn a_context_response() -> serde_json::Value {
    serde_json::json!({
        "choiceContextData": {
            "values": { "amulet-rules": { "tag": "AV_ContractId", "value": "00rules" } }
        },
        "disclosedContracts": [{
            "templateId": "pkgA:Splice.AmuletRules:AmuletRules",
            "contractId": "00rules",
            "createdEventBlob": "AQID",
            "synchronizerId": "sync::1220ab"
        }]
    })
}

/// The exercise a workflow built: the contract, the choice, and the argument
/// rendered so the context can be looked for inside it.
fn exercised(command: &canton_token::TokenCommand) -> (String, String, String) {
    match command.command().command.as_ref().expect("a command") {
        PbCommand::Exercise(e) => (
            e.contract_id.clone(),
            e.choice.clone(),
            format!("{:?}", e.choice_argument),
        ),
        other => panic!("expected an exercise, got {other:?}"),
    }
}

/// What every workflow must get right: the exercise names the contract and
/// choice, the registry's context is *inside* the argument, and the contracts
/// the registry said to disclose travel with the command.
fn assert_built(
    command: &canton_token::TokenCommand,
    contract_id: &str,
    choice: &str,
    disclosed: usize,
) {
    let (cid, name, argument) = exercised(command);
    assert_eq!(cid, contract_id);
    assert_eq!(name, choice);
    assert!(
        argument.contains("amulet-rules") && argument.contains("00rules"),
        "{choice}: the registry's context must be in the choice argument: {argument}"
    );
    assert_eq!(command.disclosed_contracts().len(), disclosed, "{choice}");
    assert_eq!(command.disclosed_contracts()[0].contract_id, "00rules");
}

fn v2_account(owner: &str) -> h2::Account {
    h2::Account {
        owner: Some(party(owner)),
        provider: None,
        id: String::new(),
    }
}

fn a_v2_transfer() -> ti2::Transfer {
    ti2::Transfer {
        sender: v2_account("alice::1220ab"),
        receiver: v2_account("bob::1220cd"),
        amount: "10.5".parse().expect("a numeric"),
        instrument_id: h2::InstrumentId {
            admin: party("dso::1220ef"),
            id: "Amulet".to_string(),
        },
        requested_at: rt::Timestamp(1_700_000_000_000_000),
        execute_before: rt::Timestamp(1_700_000_600_000_000),
        input_holding_cids: vec![rt::ContractId::new("00holding")],
        meta: no_meta(),
    }
}

fn a_v2_settlement() -> al2::SettlementInfo {
    al2::SettlementInfo {
        executors: vec![party("exec::1220ff")],
        id: "dvp-1".to_string(),
        cid: None,
        meta: no_meta(),
    }
}

fn a_v2_allocation() -> al2::AllocationSpecification {
    al2::AllocationSpecification {
        admin: party("dso::1220ef"),
        authorizer: v2_account("alice::1220ab"),
        transfer_leg_sides: vec![al2::TransferLegSide {
            transfer_leg_id: "leg-1".to_string(),
            side: al2::TransferSide::SenderSide,
            otherside: v2_account("bob::1220cd"),
            amount: "10.5".parse().expect("a numeric"),
            instrument_id: "Amulet".to_string(),
            meta: no_meta(),
        }],
        settlement_deadline: Some(rt::Timestamp(1_700_003_600_000_000)),
        next_iteration_funding: None,
        committed: false,
        meta: no_meta(),
    }
}

fn a_v1_allocation() -> al1::AllocationSpecification {
    al1::AllocationSpecification {
        settlement: al1::SettlementInfo {
            executor: party("exec::1220ff"),
            settlement_ref: al1::Reference {
                id: "dvp-1".to_string(),
                cid: None,
            },
            requested_at: rt::Timestamp(1_700_000_000_000_000),
            allocate_before: rt::Timestamp(1_700_000_600_000_000),
            settle_before: rt::Timestamp(1_700_003_600_000_000),
            meta: no_meta(),
        },
        transfer_leg_id: "leg-1".to_string(),
        transfer_leg: al1::TransferLeg {
            sender: party("alice::1220ab"),
            receiver: party("bob::1220cd"),
            amount: "10.5".parse().expect("a numeric"),
            instrument_id: h::InstrumentId {
                admin: party("dso::1220ef"),
                id: "Amulet".to_string(),
            },
            meta: no_meta(),
        },
    }
}

#[tokio::test]
async fn a_v2_transfer_names_its_actors_and_carries_the_context() {
    let (base, recorded) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");

    let command =
        canton_token::v2::transfer(&client, a_v2_transfer(), vec![party("alice::1220ab")])
            .await
            .expect("resolves");

    let seen = recorded.lock().expect("lock").clone();
    assert_eq!(seen.method, "POST");
    assert_eq!(
        seen.path,
        "/registry/transfer-instruction/v2/transfer-factory"
    );
    let arguments = &seen.body["choiceArguments"];
    assert_eq!(arguments["actors"], serde_json::json!(["alice::1220ab"]));
    assert_eq!(arguments["transfer"]["sender"]["owner"], "alice::1220ab");
    assert_eq!(arguments["transfer"]["receiver"]["owner"], "bob::1220cd");
    assert_eq!(arguments["transfer"]["amount"], "10.5");
    assert_eq!(
        arguments["extraArgs"],
        serde_json::json!({ "context": { "values": {} }, "meta": { "values": {} } })
    );

    assert_built(&command, "00factory", "TransferFactory_Transfer", 2);
    assert_eq!(
        command.transfer_kind(),
        Some(canton_token::TransferKind::Direct)
    );
}

#[tokio::test]
async fn the_v2_transfer_instruction_choices_each_ask_their_own_context() {
    let instruction = rt::ContractId::<ti2::TransferInstruction>::new("00ti");
    let actors = || vec![party("bob::1220cd")];
    for (name, expected_path, expected_choice) in [
        ("accept", "accept", "TransferInstruction_Accept"),
        ("reject", "reject", "TransferInstruction_Reject"),
        ("withdraw", "withdraw", "TransferInstruction_Withdraw"),
    ] {
        let (base, recorded) = registry(a_context_response()).await;
        let client = RegistryClient::new(&base).expect("client");
        let command = match name {
            "accept" => canton_token::v2::accept(&client, &instruction, actors()).await,
            "reject" => canton_token::v2::reject(&client, &instruction, actors()).await,
            _ => canton_token::v2::withdraw(&client, &instruction, actors()).await,
        }
        .expect(name);

        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{name}");
        assert_eq!(
            seen.path,
            format!("/registry/transfer-instruction/v2/00ti/choice-contexts/{expected_path}")
        );
        assert_eq!(
            seen.body,
            serde_json::json!({ "excludeDebugFields": true }),
            "{name}"
        );
        assert_built(&command, "00ti", expected_choice, 1);
        let (_, _, argument) = exercised(&command);
        assert!(
            argument.contains("bob::1220cd"),
            "{name}: the actors are in the argument"
        );
    }
}

#[tokio::test]
async fn a_v2_allocation_names_its_settlement_when_it_is_created() {
    let (base, recorded) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");

    let command = canton_token::v2::allocate(
        &client,
        a_v2_settlement(),
        a_v2_allocation(),
        rt::Timestamp(1_700_000_000_000_000),
        vec![rt::ContractId::new("00holding")],
        vec![party("alice::1220ab")],
    )
    .await
    .expect("resolves");

    let seen = recorded.lock().expect("lock").clone();
    assert_eq!(
        seen.path,
        "/registry/allocation-instruction/v2/allocation-factory"
    );
    let arguments = &seen.body["choiceArguments"];
    assert_eq!(arguments["settlement"]["id"], "dvp-1");
    assert_eq!(
        arguments["settlement"]["executors"],
        serde_json::json!(["exec::1220ff"])
    );
    assert_eq!(
        arguments["allocation"]["transferLegSides"][0]["transferLegId"],
        "leg-1"
    );
    assert_eq!(
        arguments["inputHoldingCids"],
        serde_json::json!(["00holding"])
    );
    assert_eq!(arguments["actors"], serde_json::json!(["alice::1220ab"]));

    assert_built(&command, "00factory", "AllocationFactory_Allocate", 2);
}

#[tokio::test]
async fn a_v2_settlement_goes_through_the_settlement_factory_as_a_batch() {
    let (base, recorded) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");

    let legs = vec![al2::TransferLeg {
        transfer_leg_id: "leg-1".to_string(),
        sender: v2_account("alice::1220ab"),
        receiver: v2_account("bob::1220cd"),
        amount: "10.5".parse().expect("a numeric"),
        instrument_id: "Amulet".to_string(),
        meta: no_meta(),
    }];
    let allocations = vec![al2::FinalizedAllocation {
        allocation_cid: rt::ContractId::new("00alloc"),
        extra_transfer_leg_sides: Vec::new(),
        next_iteration_funding: None,
    }];
    let command = canton_token::v2::settle_batch(
        &client,
        a_v2_settlement(),
        legs,
        allocations,
        vec![party("exec::1220ff")],
    )
    .await
    .expect("resolves");

    let seen = recorded.lock().expect("lock").clone();
    assert_eq!(seen.path, "/registry/allocation/v2/settlement-factory");
    let arguments = &seen.body["choiceArguments"];
    assert_eq!(arguments["settlement"]["id"], "dvp-1");
    assert_eq!(arguments["transferLegs"][0]["transferLegId"], "leg-1");
    assert_eq!(arguments["allocations"][0]["allocationCid"], "00alloc");
    assert_eq!(arguments["actors"], serde_json::json!(["exec::1220ff"]));

    assert_built(&command, "00factory", "SettlementFactory_SettleBatch", 2);
}

#[tokio::test]
async fn the_v2_allocation_and_instruction_choices_each_ask_their_own_context() {
    let allocation = rt::ContractId::<al2::Allocation>::new("00alloc");
    let instruction = rt::ContractId::<ai2::AllocationInstruction>::new("00ai");
    let actors = || vec![party("alice::1220ab")];
    for (name, expected_path, expected_contract, expected_choice) in [
        (
            "withdraw_allocation",
            "/registry/allocations/v2/00alloc/choice-contexts/withdraw",
            "00alloc",
            "Allocation_Withdraw",
        ),
        (
            "cancel",
            "/registry/allocations/v2/00alloc/choice-contexts/cancel",
            "00alloc",
            "Allocation_Cancel",
        ),
        (
            "accept_allocation_instruction",
            "/registry/allocation-instruction/v2/00ai/choice-contexts/accept",
            "00ai",
            "AllocationInstruction_Accept",
        ),
        (
            "withdraw_allocation_instruction",
            "/registry/allocation-instruction/v2/00ai/choice-contexts/withdraw",
            "00ai",
            "AllocationInstruction_Withdraw",
        ),
    ] {
        let (base, recorded) = registry(a_context_response()).await;
        let client = RegistryClient::new(&base).expect("client");
        let command = match name {
            "withdraw_allocation" => {
                canton_token::v2::withdraw_allocation(&client, &allocation, actors()).await
            }
            "cancel" => canton_token::v2::cancel(&client, &allocation, actors()).await,
            "accept_allocation_instruction" => {
                canton_token::v2::accept_allocation_instruction(&client, &instruction, actors())
                    .await
            }
            _ => {
                canton_token::v2::withdraw_allocation_instruction(&client, &instruction, actors())
                    .await
            }
        }
        .expect(name);

        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{name}");
        assert_eq!(seen.path, expected_path, "{name}");
        assert_eq!(
            seen.body,
            serde_json::json!({ "excludeDebugFields": true }),
            "{name}"
        );
        assert_built(&command, expected_contract, expected_choice, 1);
    }
}

#[tokio::test]
async fn a_v1_allocation_names_the_admin_and_carries_the_context() {
    let (base, recorded) = registry(a_factory_response()).await;
    let client = RegistryClient::new(&base).expect("client");

    let command = canton_token::allocation::allocate(
        &client,
        &party("dso::1220ef"),
        a_v1_allocation(),
        rt::Timestamp(1_700_000_000_000_000),
        vec![rt::ContractId::new("00holding")],
    )
    .await
    .expect("resolves");

    let seen = recorded.lock().expect("lock").clone();
    assert_eq!(seen.method, "POST");
    assert_eq!(
        seen.path,
        "/registry/allocation-instruction/v1/allocation-factory"
    );
    let arguments = &seen.body["choiceArguments"];
    assert_eq!(arguments["expectedAdmin"], "dso::1220ef");
    assert_eq!(
        arguments["allocation"]["settlement"]["executor"],
        "exec::1220ff"
    );
    assert_eq!(arguments["allocation"]["transferLeg"]["amount"], "10.5");
    assert_eq!(
        arguments["inputHoldingCids"],
        serde_json::json!(["00holding"])
    );

    assert_built(&command, "00factory", "AllocationFactory_Allocate", 2);
}

#[tokio::test]
async fn the_v1_allocation_choices_each_ask_their_own_context() {
    let allocation = rt::ContractId::<al1::Allocation>::new("00alloc");
    for (name, expected_path, expected_choice) in [
        (
            "execute_transfer",
            "execute-transfer",
            "Allocation_ExecuteTransfer",
        ),
        ("withdraw", "withdraw", "Allocation_Withdraw"),
        ("cancel", "cancel", "Allocation_Cancel"),
    ] {
        let (base, recorded) = registry(a_context_response()).await;
        let client = RegistryClient::new(&base).expect("client");
        let command = match name {
            "execute_transfer" => {
                canton_token::allocation::execute_transfer(&client, &allocation).await
            }
            "withdraw" => canton_token::allocation::withdraw(&client, &allocation).await,
            _ => canton_token::allocation::cancel(&client, &allocation).await,
        }
        .expect(name);

        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(seen.method, "POST", "{name}");
        assert_eq!(
            seen.path,
            format!("/registry/allocations/v1/00alloc/choice-contexts/{expected_path}")
        );
        assert_built(&command, "00alloc", expected_choice, 1);
    }
}

#[tokio::test]
async fn the_v1_transfer_instruction_choices_each_ask_their_own_context() {
    let instruction = rt::ContractId::<ti::TransferInstruction>::new("00ti");
    for (name, expected_choice) in [
        ("accept", "TransferInstruction_Accept"),
        ("reject", "TransferInstruction_Reject"),
        ("withdraw", "TransferInstruction_Withdraw"),
    ] {
        let (base, recorded) = registry(a_context_response()).await;
        let client = RegistryClient::new(&base).expect("client");
        let command = match name {
            "accept" => canton_token::accept(&client, &instruction).await,
            "reject" => canton_token::reject(&client, &instruction).await,
            _ => canton_token::withdraw(&client, &instruction).await,
        }
        .expect(name);

        let seen = recorded.lock().expect("lock").clone();
        assert_eq!(
            seen.path,
            format!("/registry/transfer-instruction/v1/00ti/choice-contexts/{name}")
        );
        assert_built(&command, "00ti", expected_choice, 1);
        assert_eq!(
            command.transfer_kind(),
            None,
            "{name}: a choice on an instruction has no kind"
        );
    }
}

/// A context exposes the data the registry returned, before it is decoded
/// into the generated type: what a caller logs, or inspects for a value the
/// choice does not carry.
#[tokio::test]
async fn a_choice_context_exposes_the_registrys_data() {
    let (base, _) = registry(a_context_response()).await;
    let context = RegistryClient::new(&base)
        .expect("client")
        .transfer_instruction_context(
            "00ti",
            canton_token::TransferInstructionChoice::Accept,
            &canton_token::ChoiceContextRequest::default(),
        )
        .await
        .expect("resolves");

    assert_eq!(
        context.data(),
        &serde_json::json!({
            "values": { "amulet-rules": { "tag": "AV_ContractId", "value": "00rules" } }
        })
    );
}

/// Serve one response with the status and content type given, recording
/// nothing: for the answers the standard's happy path never gives.
async fn serve_once(status: u16, reason: &str, content_type: &str, body: &str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let http = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let mut chunk = [0u8; 4096];
        let _ = socket.read(&mut chunk).await;
        socket.write_all(http.as_bytes()).await.expect("write");
    });
    format!("http://127.0.0.1:{port}")
}

/// `instrument()` has three answers and they must stay distinct: issued,
/// not issued (the registry's own 404), and "this is not a registry" (a 404
/// from something else, which must not read as "not issued").
#[tokio::test]
async fn an_instrument_is_some_none_or_an_error_depending_on_who_answered() {
    let (base, recorded) = registry(serde_json::json!({
        "id": "Amulet", "name": "Canton Coin", "symbol": "CC", "decimals": 10,
        "supportedApis": { "splice-api-token-holding-v1": 1 }
    }))
    .await;
    let issued = RegistryClient::new(&base)
        .expect("client")
        .instrument("Amulet")
        .await
        .expect("resolves")
        .expect("issued");
    assert_eq!(issued.id, "Amulet");
    assert_eq!(issued.name, "Canton Coin");
    assert_eq!(issued.decimals, 10);
    assert_eq!(
        issued.supported_apis.get("splice-api-token-holding-v1"),
        Some(&1)
    );
    assert!(!issued.paused);
    let seen = recorded.lock().expect("lock").clone();
    assert_eq!(seen.method, "GET");
    assert_eq!(seen.path, "/registry/metadata/v1/instruments/Amulet");

    for body in [r#"{"error":"no such instrument"}"#, ""] {
        let base = serve_once(404, "Not Found", "application/json", body).await;
        let none = RegistryClient::new(&base)
            .expect("client")
            .instrument("Nope")
            .await
            .expect("a registry's own 404 is an answer");
        assert!(none.is_none(), "{body:?}");
    }

    let base = serve_once(
        404,
        "Not Found",
        "text/html",
        "<html><body>nginx</body></html>",
    )
    .await;
    let error = RegistryClient::new(&base)
        .expect("client")
        .instrument("Amulet")
        .await
        .expect_err("a proxy's 404 is not 'not issued'");
    assert!(
        matches!(&error, canton_core::Error::Http { status: 404, body, .. } if body.contains("nginx")),
        "{error:?}"
    );
}
