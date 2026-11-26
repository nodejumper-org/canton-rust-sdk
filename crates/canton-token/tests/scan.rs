//! The Scan client, against a Scan standing in this process.
//!
//! A real Scan belongs to a Super Validator, and a LocalNet running only a
//! validator has none, so without this the HTTP layer would be exercised
//! nowhere. The stub records what it was asked: the method, the path, the
//! token and the body, which is the half Splice's specification is precise
//! about.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

use canton_core::Error;
use canton_token::scan::{HistoryPosition, ScanClient, SnapshotRequest};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

/// One request the stub received.
#[derive(Clone, Debug, Default)]
struct Asked {
    method: String,
    /// The path with its query, as sent.
    path: String,
    authorization: Option<String>,
    body: Value,
}

/// A stub that answers each request with the next of `replies` (a status and
/// a body), and with `200 {}` once they run out. Returns its base URL and
/// what it was asked.
async fn stub(replies: Vec<(u16, String)>) -> (String, Arc<Mutex<Vec<Asked>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let asked = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&asked);
    let replies = Arc::new(Mutex::new(VecDeque::from(replies)));

    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 4096];
            // Read until the headers are complete, then until the body is.
            let request = loop {
                let read = socket.read(&mut chunk).await.expect("read");
                if read == 0 {
                    break None;
                }
                buffer.extend_from_slice(&chunk[..read]);
                let text = String::from_utf8_lossy(&buffer).to_string();
                let Some(head_end) = text.find("\r\n\r\n") else {
                    continue;
                };
                let head = &text[..head_end];
                let body = &text[head_end + 4..];
                let header = |wanted: &str| {
                    head.lines().find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case(wanted)
                            .then(|| value.trim().to_string())
                    })
                };
                let content_length: usize = header("content-length")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0);
                if body.len() < content_length {
                    continue;
                }
                let mut request_line = head.lines().next().unwrap_or_default().split_whitespace();
                break Some(Asked {
                    method: request_line.next().unwrap_or_default().to_string(),
                    path: request_line.next().unwrap_or_default().to_string(),
                    authorization: header("authorization"),
                    body: serde_json::from_str(body).unwrap_or(Value::Null),
                });
            };
            let Some(request) = request else { continue };
            seen.lock().expect("lock").push(request);

            let (status, payload) = replies
                .lock()
                .expect("lock")
                .pop_front()
                .unwrap_or((200, "{}".to_string()));
            let http = format!(
                "HTTP/1.1 {status} Stub\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
                payload.len()
            );
            socket.write_all(http.as_bytes()).await.expect("write");
            socket.flush().await.expect("flush");
            let _ = socket.shutdown().await;
        }
    });

    (format!("http://127.0.0.1:{port}"), asked)
}

fn ok(body: &Value) -> (u16, String) {
    (200, body.to_string())
}

/// A Scan at `<stub>/api/scan` and a validator at `<stub>/api/validator`.
fn scan(base: &str) -> ScanClient {
    ScanClient::new(&format!("{base}/api/scan")).expect("a Scan client")
}

fn validator(base: &str) -> ScanClient {
    ScanClient::through_validator(&format!("{base}/api/validator"))
        .expect("a scan-proxy client")
        .with_token("the-validator-token")
}

fn a_contract(contract_id: &str) -> Value {
    json!({
        "template_id": "pkg:Splice.AmuletRules:AmuletRules",
        "contract_id": contract_id,
        "payload": { "dso": "DSO::1220" },
        "created_event_blob": "AQID",
        "created_at": "2026-11-26T10:00:00.000000Z",
    })
}

fn with_state(contract_id: &str) -> Value {
    json!({ "contract": a_contract(contract_id), "domain_id": "global-domain::1220aa" })
}

/// The paths a document publishes, each with the HTTP methods it takes.
fn published(document: &str) -> BTreeMap<String, Vec<String>> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../testdata/openapi")
        .join(document);
    let text = std::fs::read_to_string(&path).expect(document);
    let mut paths: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut in_paths = false;
    let mut current = None;
    for line in text.lines() {
        if line.starts_with("paths:") {
            in_paths = true;
            continue;
        }
        if in_paths && !line.is_empty() && !line.starts_with(' ') {
            in_paths = false;
        }
        if !in_paths {
            continue;
        }
        if line.starts_with("  /") && line.trim_end().ends_with(':') {
            let path = normalise(line.trim().trim_end_matches(':'));
            paths.entry(path.clone()).or_default();
            current = Some(path);
        } else if let Some(method) = line
            .strip_prefix("    ")
            .and_then(|rest| rest.strip_suffix(':'))
            .filter(|method| ["get", "post", "put", "delete"].contains(method))
        {
            let path = current.clone().expect("a method under a path");
            paths
                .get_mut(&path)
                .expect("the path")
                .push(method.to_uppercase());
        }
    }
    paths
}

/// `{party}`, `{domain_id}`, … all become `{}`: the name of a path parameter
/// is documentation, its position is the contract.
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

/// What the stub was asked, as the specification would write it: the base
/// path and the query removed, and the ids the tests pass replaced by `{}`.
fn as_specified(asked: &Asked, base_path: &str) -> (String, String) {
    let path = asked.path.split('?').next().unwrap_or_default();
    let path = path.strip_prefix(base_path).expect("under the base path");
    let path = path
        .split('/')
        .map(|segment| {
            if segment.starts_with("ID-") {
                "{}"
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join("/");
    (asked.method.clone(), path)
}

/// Call every method the client has, ignoring what comes back: the stub
/// answers `{}`, which most of them cannot decode, and the request is what
/// is being looked at.
async fn call_everything(client: &ScanClient) {
    let snapshot = SnapshotRequest::at(4, "2026-11-26T10:00:00Z").for_parties(["ID-owner"]);
    let after = HistoryPosition {
        migration_id: 4,
        record_time: "2026-11-26T10:00:00Z".to_string(),
    };
    let _ = client.dso_party_id().await;
    let _ = client.dso().await;
    let _ = client.scans().await;
    let _ = client.open_and_issuing_mining_rounds().await;
    let _ = client.closed_rounds().await;
    let _ = client.amulet_rules().await;
    let _ = client.featured_app_right("ID-provider").await;
    let _ = client.transfer_preapproval_by_party("ID-party").await;
    let _ = client
        .member_traffic_status("ID-synchronizer", "ID-member")
        .await;
    let _ = client.ans_entry_by_party("ID-party").await;
    let _ = client.ans_entry_by_name("ID-name").await;
    let _ = client.ans_entries(Some("a"), 10).await;
    let _ = client.updates(Some(&after), 10).await;
    let _ = client.update("ID-update").await;
    let _ = client.events(None, 10).await;
    let _ = client
        .acs_snapshot_timestamp("2026-11-26T10:00:00Z", 4)
        .await;
    let _ = client.acs_snapshot(&snapshot).await;
    let _ = client.holdings_state(&snapshot).await;
    let _ = client.holdings_summary(&snapshot).await;
}

/// Every path the client asks a Scan for, with the method it uses, is one
/// Splice's `scan.yaml` publishes, at the revision vendored under
/// `testdata/openapi/`. And every one of the client's methods reaches the
/// Scan: nothing is refused once internal endpoints are opted into.
#[tokio::test]
async fn every_scan_path_and_method_is_the_one_splice_publishes() {
    let (base, asked) = stub(Vec::new()).await;
    call_everything(&scan(&base).with_internal_endpoints()).await;

    let published = published("scan.yaml");
    assert!(
        published.len() >= 60,
        "scan.yaml lists {} paths",
        published.len()
    );
    let asked = asked.lock().expect("lock").clone();
    assert_eq!(asked.len(), 19, "one request per method: {asked:#?}");
    for request in &asked {
        let (method, path) = as_specified(request, "/api/scan");
        let methods = published
            .get(&path)
            .unwrap_or_else(|| panic!("scan.yaml does not publish {path}"));
        assert!(
            methods.contains(&method),
            "scan.yaml publishes {path} for {methods:?}, not {method}"
        );
        assert_eq!(
            request.authorization, None,
            "{path}: no token was configured"
        );
    }
}

/// Through a validator the same calls go under its `scan-proxy` prefix, with
/// the method `scan-proxy.yaml` publishes (two of them differ from Scan's),
/// and with the token on every one. What the proxy does not serve is refused
/// before anything is sent.
#[tokio::test]
async fn every_scan_proxy_path_and_method_is_the_one_splice_publishes() {
    let (base, asked) = stub(Vec::new()).await;
    call_everything(&validator(&base)).await;

    let published = published("scan-proxy.yaml");
    assert!(
        published.len() >= 10,
        "scan-proxy.yaml lists {} paths",
        published.len()
    );
    let asked = asked.lock().expect("lock").clone();
    let mut served = Vec::new();
    for request in &asked {
        let (method, path) = as_specified(request, "/api/validator");
        let methods = published
            .get(&path)
            .unwrap_or_else(|| panic!("scan-proxy.yaml does not publish {path}"));
        assert!(
            methods.contains(&method),
            "scan-proxy.yaml publishes {path} for {methods:?}, not {method}"
        );
        assert_eq!(
            request.authorization.as_deref(),
            Some("Bearer the-validator-token"),
            "{path}"
        );
        served.push(format!("{method} {path}"));
    }
    assert_eq!(
        served,
        [
            "GET /v0/scan-proxy/dso-party-id",
            "GET /v1/scan-proxy/dso",
            "GET /v0/scan-proxy/open-and-issuing-mining-rounds",
            "GET /v0/scan-proxy/amulet-rules",
            "GET /v0/scan-proxy/featured-apps/{}",
            "GET /v0/scan-proxy/transfer-preapprovals/by-party/{}",
            "GET /v0/scan-proxy/ans-entries/by-party/{}",
            "GET /v0/scan-proxy/ans-entries/by-name/{}",
            "GET /v0/scan-proxy/ans-entries",
            "POST /v1/scan-proxy/holdings/summary",
        ]
    );
}

/// The reads a validator does not proxy say so, and say where to ask.
#[tokio::test]
async fn what_a_validator_does_not_proxy_is_refused_before_it_is_sent() {
    let (base, asked) = stub(Vec::new()).await;
    let client = validator(&base);
    let snapshot = SnapshotRequest::at(4, "2026-11-26T10:00:00Z").for_parties(["owner"]);

    let refusals = [
        client.scans().await.map(|_| ()),
        client.closed_rounds().await.map(|_| ()),
        client.member_traffic_status("s", "m").await.map(|_| ()),
        client.updates(None, 10).await.map(|_| ()),
        client.update("u").await.map(|_| ()),
        client.events(None, 10).await.map(|_| ()),
        client.acs_snapshot_timestamp("t", 4).await.map(|_| ()),
        client.acs_snapshot(&snapshot).await.map(|_| ()),
        client.holdings_state(&snapshot).await.map(|_| ()),
    ];
    for refusal in refusals {
        let error = refusal.expect_err("not served by the scan-proxy");
        assert!(matches!(error, Error::InvalidRequest(_)), "{error:?}");
        assert!(error.to_string().contains("ScanClient::new"), "{error}");
    }
    assert!(asked.lock().expect("lock").is_empty(), "nothing was sent");
}

/// Three calls reach endpoints Splice tags `internal` on a Scan. They are
/// refused until the caller accepts that, and never needed that through a
/// validator, whose proxy serves them as its own API.
#[tokio::test]
async fn a_scans_internal_endpoints_need_the_caller_to_opt_in() {
    let (base, asked) = stub(Vec::new()).await;
    let client = scan(&base);
    for refusal in [
        client.amulet_rules().await.map(|_| ()),
        client.featured_app_right("provider").await.map(|_| ()),
        client
            .transfer_preapproval_by_party("party")
            .await
            .map(|_| ()),
    ] {
        let error = refusal.expect_err("an internal endpoint, not opted into");
        assert!(matches!(error, Error::InvalidRequest(_)), "{error:?}");
        let message = error.to_string();
        assert!(
            message.contains("with_internal_endpoints") && message.contains("through_validator"),
            "{message}"
        );
    }
    assert!(asked.lock().expect("lock").is_empty(), "nothing was sent");

    // The external endpoints never needed it.
    let (base, asked) = stub(vec![ok(&json!({ "dso_party_id": "DSO::1220" }))]).await;
    assert_eq!(scan(&base).dso_party_id().await.unwrap(), "DSO::1220");
    assert_eq!(asked.lock().expect("lock").len(), 1);
}

/// A Scan and a validator answer the same two questions in different shapes:
/// a Scan leaves out what the caller says it has cached and keys the rest by
/// contract id, the proxy lists everything. The client returns one shape.
#[tokio::test]
async fn rounds_and_rules_read_the_same_from_a_scan_and_from_a_validator() {
    let from_scan = json!({
        "time_to_live_in_microseconds": 600_000_000,
        "open_mining_rounds": {
            "00open1": { "contract": a_contract("00open1"), "domain_id": "global-domain::1220aa" },
            "00open2": { "contract": a_contract("00open2") },
        },
        "issuing_mining_rounds": {
            "00issuing": { "contract": a_contract("00issuing"), "domain_id": "global-domain::1220aa" },
        },
    });
    let (base, asked) = stub(vec![
        ok(&from_scan),
        ok(&json!({ "amulet_rules_update": with_state("00rules") })),
    ])
    .await;
    let client = scan(&base).with_internal_endpoints();
    let rounds = client.open_and_issuing_mining_rounds().await.unwrap();
    let ids = |rounds: &[canton_token::scan::ContractWithState]| {
        rounds
            .iter()
            .map(|round| round.contract.contract_id.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(ids(&rounds.open), ["00open1", "00open2"]);
    assert_eq!(ids(&rounds.issuing), ["00issuing"]);
    assert_eq!(rounds.open[1].synchronizer_id, None, "in transit");
    let rules = client.amulet_rules().await.unwrap();
    assert_eq!(rules.contract.contract_id, "00rules");
    assert_eq!(rules.disclosed().unwrap().created_event_blob, [1, 2, 3]);
    // The Scan was told nothing is cached, so it left nothing out.
    let asked = asked.lock().expect("lock").clone();
    assert_eq!(
        asked[0].body,
        json!({
            "cached_open_mining_round_contract_ids": [],
            "cached_issuing_round_contract_ids": [],
        })
    );
    assert_eq!(asked[1].body, json!({}));

    let (base, _) = stub(vec![
        ok(&json!({
            "open_mining_rounds": [with_state("00open1"), with_state("00open2")],
            "issuing_mining_rounds": [with_state("00issuing")],
        })),
        ok(&json!({ "amulet_rules": with_state("00rules") })),
    ])
    .await;
    let client = validator(&base);
    let rounds = client.open_and_issuing_mining_rounds().await.unwrap();
    assert_eq!(ids(&rounds.open), ["00open1", "00open2"]);
    assert_eq!(ids(&rounds.issuing), ["00issuing"]);
    assert_eq!(
        client.amulet_rules().await.unwrap().contract.contract_id,
        "00rules"
    );

    // A round or the rules left out as cached is not something this client
    // can have asked for.
    let (base, _) = stub(vec![
        ok(&json!({
            "time_to_live_in_microseconds": 1,
            "open_mining_rounds": { "00open1": {} },
            "issuing_mining_rounds": {},
        })),
        ok(&json!({ "amulet_rules_update": {} })),
    ])
    .await;
    let client = scan(&base).with_internal_endpoints();
    let error = client.open_and_issuing_mining_rounds().await.unwrap_err();
    assert!(error.to_string().contains("00open1"), "{error}");
    let error = client.amulet_rules().await.unwrap_err();
    assert!(matches!(error, Error::UnexpectedResponse(_)), "{error:?}");
}

#[tokio::test]
async fn typed_replies_carry_what_scan_said() {
    let (base, _) = stub(vec![
        ok(&json!({
            "dso_party_id": "DSO::1220",
            "voting_threshold": 9,
            "latest_mining_round": with_state("00round"),
            "amulet_rules": with_state("00rules"),
            "dso_rules": with_state("00dso"),
            "sv_node_states": [with_state("00sv1")],
            "sv_user": "a field this client does not read",
        })),
        ok(&json!({
            "scans": [{
                "domainId": "global-domain::1220aa",
                "scans": [{ "publicUrl": "https://scan.sv-1.example", "svName": "SV-1" }],
            }],
        })),
        ok(&json!({
            "traffic_status": {
                "actual": { "total_consumed": 1200, "total_limit": 200_000 },
                "target": { "total_purchased": 400_000 },
            },
        })),
        ok(&json!({ "featured_app_right": a_contract("00right") })),
        ok(&json!({})),
    ])
    .await;
    let client = scan(&base).with_internal_endpoints();

    let dso = client.dso().await.unwrap();
    assert_eq!(dso.dso_party_id, "DSO::1220");
    assert_eq!(dso.voting_threshold, 9);
    assert_eq!(dso.sv_node_states.len(), 1);
    assert_eq!(dso.initial_round, None);

    let scans = client.scans().await.unwrap();
    assert_eq!(scans[0].synchronizer_id, "global-domain::1220aa");
    assert_eq!(scans[0].scans[0].public_url, "https://scan.sv-1.example");
    assert_eq!(scans[0].scans[0].sv_name, "SV-1");

    let traffic = client
        .member_traffic_status("global-domain::1220aa", "PAR::validator::1220")
        .await
        .unwrap();
    assert_eq!(
        (
            traffic.total_consumed,
            traffic.total_limit,
            traffic.total_purchased
        ),
        (1200, 200_000, 400_000)
    );

    let right = client.featured_app_right("provider::1220").await.unwrap();
    assert_eq!(right.unwrap().contract_id, "00right");
    assert!(
        client
            .featured_app_right("nobody::1220")
            .await
            .unwrap()
            .is_none(),
        "a provider that is not featured has no right"
    );
}

/// The snapshot reads send what the request says and nothing else, and a
/// holdings read with no owner is refused where the message can say so.
#[tokio::test]
async fn snapshot_reads_send_what_the_request_says() {
    let (base, asked) = stub(vec![
        ok(&json!({
            "record_time": "2026-11-26T06:00:00Z",
            "migration_id": 4,
            "created_events": [{
                "contract_id": "00amulet",
                "template_id": "pkg:Splice.Amulet:Amulet",
                "package_name": "splice-amulet",
                "create_arguments": { "owner": "alice::1220" },
                "created_at": "2026-11-25T10:00:00Z",
                "signatories": ["DSO::1220", "alice::1220"],
                "observers": [],
            }],
            "next_page_token": "42",
        })),
        ok(&json!({
            "record_time": "2026-11-26T06:00:00Z",
            "migration_id": 4,
            "summaries": [{
                "party_id": "alice::1220",
                "total_unlocked_coin": "10.5000000000",
                "total_locked_coin": "0.0000000000",
                "total_coin_holdings": "10.5000000000",
            }],
        })),
    ])
    .await;
    let client = scan(&base);

    let request = SnapshotRequest::at(4, "2026-11-26T08:00:00Z")
        .at_or_before()
        .with_page_size(500)
        .after("41")
        .for_parties(["alice::1220"])
        .for_templates(["splice-amulet:Splice.Amulet:Amulet"]);
    let snapshot = client.acs_snapshot(&request).await.unwrap();
    assert_eq!(snapshot.created_events[0].contract_id, "00amulet");
    assert_eq!(snapshot.next_page_token.as_deref(), Some("42"));

    let summaries = client.holdings_summary(&request).await.unwrap();
    assert_eq!(summaries.summaries[0].total_coin_holdings, "10.5000000000");

    let asked = asked.lock().expect("lock").clone();
    assert_eq!(
        asked[0].body,
        json!({
            "migration_id": 4,
            "record_time": "2026-11-26T08:00:00Z",
            "record_time_match": "at_or_before",
            "page_size": 500,
            "after": "41",
            "party_ids": ["alice::1220"],
            "templates": ["splice-amulet:Splice.Amulet:Amulet"],
        })
    );
    assert_eq!(
        asked[1].body,
        json!({
            "migration_id": 4,
            "record_time": "2026-11-26T08:00:00Z",
            "record_time_match": "at_or_before",
            "owner_party_ids": ["alice::1220"],
        })
    );

    // A holdings read with no owner is refused here, where the message can
    // say what is missing.
    let no_owner = SnapshotRequest::at(4, "2026-11-26T08:00:00Z");
    for refusal in [
        client.holdings_state(&no_owner).await.map(|_| ()),
        client.holdings_summary(&no_owner).await.map(|_| ()),
    ] {
        let error = refusal.expect_err("no owner");
        assert!(error.to_string().contains("for_parties"), "{error}");
    }
}

/// A party, a name or an id goes into the path as one segment, whatever is in
/// it: it cannot address another resource or add a query.
#[tokio::test]
async fn an_id_cannot_change_which_resource_is_asked_for() {
    let (base, asked) = stub(Vec::new()).await;
    let client = validator(&base);
    let _ = client.ans_entry_by_name("../../admin?x=1#y").await;
    let _ = client.featured_app_right("a/b").await;
    let _ = client.ans_entries(Some("a&page_size=1000"), 10).await;
    let _ = client
        .get_json(
            "v0",
            &["transfer-command", "status"],
            &[("sender", "a&b"), ("nonce", "1")],
        )
        .await;
    let asked = asked.lock().expect("lock").clone();
    assert_eq!(
        asked[0].path,
        "/api/validator/v0/scan-proxy/ans-entries/by-name/..%2F..%2Fadmin%3Fx=1%23y"
    );
    assert_eq!(
        asked[1].path,
        "/api/validator/v0/scan-proxy/featured-apps/a%2Fb"
    );
    assert_eq!(
        asked[2].path,
        "/api/validator/v0/scan-proxy/ans-entries?name_prefix=a%26page_size%3D1000&page_size=10"
    );
    assert_eq!(
        asked[3].path,
        "/api/validator/v0/scan-proxy/transfer-command/status?sender=a%26b&nonce=1"
    );
}

/// "No such entry" is an answer, and Scan gives it as a `404` with its error
/// body. A `404` from anything else is a wrong base URL or a gateway, and
/// must not read as "no such entry".
#[tokio::test]
async fn a_404_is_none_only_when_scan_is_the_one_saying_it() {
    let (base, _) = stub(vec![
        (
            404,
            json!({ "error": "No ans entry found for party: alice" }).to_string(),
        ),
        (
            404,
            json!({ "error": "No TransferPreapproval found for party: alice" }).to_string(),
        ),
        (
            404,
            json!({ "error": "Transaction with update id u not found" }).to_string(),
        ),
        (
            404,
            json!({ "error": "No snapshots found before t" }).to_string(),
        ),
    ])
    .await;
    let client = scan(&base).with_internal_endpoints();
    assert!(client.ans_entry_by_party("alice").await.unwrap().is_none());
    assert!(
        client
            .transfer_preapproval_by_party("alice")
            .await
            .unwrap()
            .is_none()
    );
    assert!(client.update("u").await.unwrap().is_none());
    assert!(
        client
            .acs_snapshot_timestamp("t", 4)
            .await
            .unwrap()
            .is_none()
    );

    for body in ["<html><body>404 Not Found</body></html>", "", "{}"] {
        let (base, _) = stub(vec![(404, body.to_string())]).await;
        let error = scan(&base)
            .ans_entry_by_party("alice")
            .await
            .expect_err("not Scan's own 404");
        assert!(
            matches!(error, Error::Http { status: 404, .. }),
            "{body:?}: {error:?}"
        );
    }
}

/// A Scan that fails says why, and the error carries that message and the
/// status, so the shared error model can decide whether to retry.
#[tokio::test]
async fn a_failing_scan_reports_its_own_message() {
    let (base, _) = stub(vec![
        (
            400,
            json!({ "error": "Invalid party id: nope" }).to_string(),
        ),
        (503, "upstream connect error".to_string()),
        (200, "not json".to_string()),
    ])
    .await;
    let client = scan(&base);

    let error = client.dso_party_id().await.unwrap_err();
    assert!(!error.is_retriable(), "{error:?}");
    assert!(
        error.to_string().contains("Invalid party id: nope"),
        "{error}"
    );

    let error = client.dso_party_id().await.unwrap_err();
    assert!(error.is_retriable(), "a 503 passes: {error:?}");

    let error = client.dso_party_id().await.unwrap_err();
    assert!(matches!(error, Error::UnexpectedResponse(_)), "{error:?}");
}

/// What Scan answers goes into commands the caller signs, so it has to come
/// from the Scan the caller named: a 3xx is not followed, and the redirect
/// target never hears from the client, token included.
#[tokio::test]
async fn a_redirecting_scan_is_not_followed() {
    let (target, reached) = stub(vec![ok(&json!({ "dso_party_id": "attacker::1220" }))]).await;
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 8192];
            let _ = socket.read(&mut buf).await;
            let response = format!(
                "HTTP/1.1 302 Found\r\nLocation: {target}/api/validator/v0/scan-proxy/dso-party-id\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        }
    });

    let error = validator(&format!("http://127.0.0.1:{port}"))
        .dso_party_id()
        .await
        .expect_err("a redirect is refused");
    assert!(!error.is_retriable(), "{error:?}");
    assert!(
        reached.lock().expect("lock").is_empty(),
        "the client followed the redirect"
    );
}

/// The registry routes a validator serves under its scan-proxy authenticate
/// like the rest of it, so the registry client sends the token too.
#[tokio::test]
async fn the_registry_client_sends_a_token_when_given_one() {
    let info = json!({ "adminId": "DSO::1220", "supportedApis": {} });
    let (base, asked) = stub(vec![ok(&info), ok(&info)]).await;

    let proxied = format!("{base}/api/validator/v0/scan-proxy");
    canton_token::RegistryClient::new(&proxied)
        .expect("client")
        .with_token("the-validator-token")
        .info()
        .await
        .expect("the registry describes itself");
    canton_token::RegistryClient::new(&proxied)
        .expect("client")
        .info()
        .await
        .expect("the registry describes itself");

    let asked = asked.lock().expect("lock").clone();
    assert_eq!(
        asked[0].path,
        "/api/validator/v0/scan-proxy/registry/metadata/v1/info"
    );
    assert_eq!(
        asked[0].authorization.as_deref(),
        Some("Bearer the-validator-token")
    );
    assert_eq!(asked[1].authorization, None, "no token, no header");
}
