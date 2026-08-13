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

            let path = head
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap_or_default()
                .to_string();
            *seen.lock().expect("lock") = Recorded {
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

/// A registry that fails says why, and the error carries the URL — a 404 from
/// a mistyped path and a 404 from an unknown contract read the same otherwise.
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
        assert_eq!(recorded.lock().expect("lock").path, expected);
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
        assert_eq!(recorded.lock().expect("lock").path, expected);
    }
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
